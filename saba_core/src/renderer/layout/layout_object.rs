use crate::alloc::string::ToString;
use crate::constants::CHAR_HEIGHT_WITH_PADDING;
use crate::constants::CHAR_WIDTH;
use crate::constants::CONTENT_AREA_WIDTH;
use crate::display_item::DisplayItem;
use crate::renderer::css::cssom::ComponentValue;
use crate::renderer::css::cssom::Declaration;
use crate::renderer::css::cssom::Selector;
use crate::renderer::css::cssom::StyleSheet;
use crate::renderer::dom::node::Node;
use crate::renderer::dom::node::NodeKind;
use crate::renderer::layout::computed_style::Color;
use crate::renderer::layout::computed_style::ComputedStyle;
use crate::renderer::layout::computed_style::DisplayType;
use crate::renderer::layout::computed_style::FontSize;
use alloc::rc::Rc;
use alloc::rc::Weak;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

#[derive(Debug, Clone)]
pub struct LayoutObject {
    kind: LayoutObjectKind,
    node: Rc<RefCell<Node>>,
    first_child: Option<Rc<RefCell<LayoutObject>>>,
    next_sibling: Option<Rc<RefCell<LayoutObject>>>,
    parent: Weak<RefCell<LayoutObject>>,
    style: ComputedStyle,
    point: LayoutPoint,
    size: LayoutSize,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct LayoutPoint {
    x: i64,
    y: i64,
}

impl LayoutPoint {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }

    pub fn set_x(&mut self, x: i64) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i64) {
        self.y = y;
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct LayoutSize {
    width: i64,
    height: i64,
}

impl LayoutSize {
    pub fn new(width: i64, height: i64) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> i64 {
        self.width
    }

    pub fn height(&self) -> i64 {
        self.height
    }

    pub fn set_width(&mut self, width: i64) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: i64) {
        self.height = height;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LayoutObjectKind {
    Block,
    Inline,
    Text,
}

impl LayoutObject {
    pub fn new(node: Rc<RefCell<Node>>, parent_obj: &Option<Rc<RefCell<LayoutObject>>>) -> Self {
        let parent = match parent_obj {
            Some(p) => Rc::downgrade(p),
            None => Weak::new(),
        };
        Self {
            kind: LayoutObjectKind::Block,
            node: node.clone(),
            first_child: None,
            next_sibling: None,
            parent,
            style: ComputedStyle::new(),
            point: LayoutPoint::new(0, 0),
            size: LayoutSize::new(0, 0),
        }
    }

    pub fn kind(&self) -> LayoutObjectKind {
        self.kind
    }

    pub fn node_kind(&self) -> NodeKind {
        self.node.borrow().kind().clone()
    }

    pub fn set_first_child(&mut self, first_child: Option<Rc<RefCell<LayoutObject>>>) {
        self.first_child = first_child;
    }

    pub fn first_child(&self) -> Option<Rc<RefCell<LayoutObject>>> {
        self.first_child.as_ref().cloned()
    }

    pub fn set_next_sibling(&mut self, next_sibling: Option<Rc<RefCell<LayoutObject>>>) {
        self.next_sibling = next_sibling;
    }

    pub fn next_sibling(&self) -> Option<Rc<RefCell<LayoutObject>>> {
        self.next_sibling.as_ref().cloned()
    }

    pub fn parent(&self) -> Weak<RefCell<Self>> {
        self.parent.clone()
    }

    pub fn style(&self) -> ComputedStyle {
        self.style.clone()
    }

    pub fn point(&self) -> LayoutPoint {
        self.point
    }

    pub fn size(&self) -> LayoutSize {
        self.size
    }

    pub fn is_node_selected(&self, selector: &Selector) -> bool {
        match &self.node_kind() {
            NodeKind::Element(e) => match selector {
                Selector::TypeSelector(type_name) => {
                    if e.kind().to_string() == *type_name {
                        return true;
                    }
                    false
                }
                Selector::ClassSelector(class_name) => {
                    for attr in &e.attributes() {
                        if attr.name() == "class" && attr.value() == *class_name {
                            return true;
                        }
                    }
                    false
                }
                Selector::IdSelector(id_name) => {
                    for attr in &e.attributes() {
                        if attr.name() == "id" && attr.value() == *id_name {
                            return true;
                        }
                    }
                    false
                }
                Selector::UnknownSelector => false,
            },
            _ => false,
        }
    }

    pub fn cascading_style(&mut self, declarations: Vec<Declaration>) {
        for declaration in declarations {
            match declaration.property.as_str() {
                "background-color" => {
                    if let ComponentValue::Ident(value) = &declaration.value {
                        let color = match Color::from_name(&value) {
                            Ok(color) => color,
                            Err(_) => Color::white(),
                        };
                        self.style.set_background_color(color);
                        continue;
                    }
                    if let ComponentValue::HashToken(color_code) = &declaration.value {
                        let color = match Color::from_code(&color_code) {
                            Ok(color) => color,
                            Err(_) => Color::white(),
                        };
                        self.style.set_background_color(color);
                        continue;
                    }
                }
                "color" => {
                    if let ComponentValue::Ident(value) = &declaration.value {
                        let color = match Color::from_name(&value) {
                            Ok(color) => color,
                            Err(_) => Color::black(),
                        };
                        self.style.set_color(color);
                    }

                    if let ComponentValue::HashToken(color_code) = &declaration.value {
                        let color = match Color::from_code(&color_code) {
                            Ok(color) => color,
                            Err(_) => Color::black(),
                        };
                        self.style.set_color(color);
                    }
                }
                "display" => {
                    if let ComponentValue::Ident(value) = declaration.value {
                        let display_type = match DisplayType::from_str(&value) {
                            Ok(display_type) => display_type,
                            Err(_) => DisplayType::DisplayNone,
                        };
                        self.style.set_display(display_type)
                    }
                }

                _ => {}
            }
        }
    }

    pub fn defaulting_style(
        &mut self,
        node: &Rc<RefCell<Node>>,
        parent_style: Option<ComputedStyle>,
    ) {
        self.style.defaulting(node, parent_style);
    }

    pub fn update_kind(&mut self) {
        match self.node_kind() {
            NodeKind::Document => panic!("should not create a layout object for a Document node"),
            NodeKind::Element(_) => {
                let display = self.style.display();
                match display {
                    DisplayType::Block => self.kind = LayoutObjectKind::Block,
                    DisplayType::Inline => self.kind = LayoutObjectKind::Inline,
                    DisplayType::DisplayNone => {
                        panic!("should not create a layout object for display:none")
                    }
                }
            }
            NodeKind::Text(_) => self.kind = LayoutObjectKind::Text,
        }
    }

    /// 1つのノードのサイズを計算する
    /// ユーザーによって CSS でwidth や height が司令されている場合はその値を使用するが、今回は CSS で横幅と高さは指定できないので関係ない。
    pub fn compute_size(&mut self, parent_size: LayoutSize) {
        let mut size = LayoutSize::new(0, 0);

        match self.kind() {
            LayoutObjectKind::Block => {
                // ノードがブロック要素の場合、親ノードの横幅がそのまま自身の横幅になる (本来はボックスモデルにおけるマージンやパディングを考慮するところ)
                size.set_width(parent_size.width());

                let mut height = 0;
                let mut child = self.first_child();

                let mut previous_child_kind = LayoutObjectKind::Block;
                while child.is_some() {
                    let c = match child {
                        Some(c) => c,
                        None => panic!("first child should exist"),
                    };

                    if previous_child_kind == LayoutObjectKind::Block
                        || c.borrow().kind() == LayoutObjectKind::Block
                    {
                        height += c.borrow().size.height();
                    }

                    previous_child_kind = c.borrow().kind();
                    child = c.borrow().next_sibling();
                }
                // ブロック要素の高さはすべての子ノードの高さの合計になる、インライン要素が横に並んでいる場合は、高さが増えることはない
                size.set_height(height);
            }
            LayoutObjectKind::Inline => {
                // ノードがインライン要素の場合、高さも横幅も子要素のサイズを足し合わせたものになる。本実装ではインライン要素の子ノードは常にテキストノードである想定
                let mut width = 0;
                let mut height = 0;
                let mut child = self.first_child();
                while child.is_some() {
                    let c = match child {
                        Some(c) => c,
                        None => panic!("first child should exist"),
                    };

                    width += c.borrow().size.width();
                    height += c.borrow().size.height();

                    child = c.borrow().next_sibling();
                }

                size.set_width(width);
                size.set_height(height);
            }
            LayoutObjectKind::Text => {
                if let NodeKind::Text(t) = self.node_kind() {
                    let ratio = match self.style.font_size() {
                        FontSize::Medium => 1,
                        FontSize::XLarge => 2,
                        FontSize::XXLarge => 3,
                    };
                    let width = CHAR_WIDTH * ratio * t.len() as i64;
                    if width > CONTENT_AREA_WIDTH {
                        size.set_width(CONTENT_AREA_WIDTH);
                        let line_num = if width.wrapping_rem(CONTENT_AREA_WIDTH) == 0 {
                            width.wrapping_div(CONTENT_AREA_WIDTH)
                        } else {
                            width.wrapping_div(CONTENT_AREA_WIDTH) + 1
                        };
                        size.set_height(CHAR_HEIGHT_WITH_PADDING * ratio * line_num);
                    } else {
                        size.set_width(width);
                        size.set_height(CHAR_HEIGHT_WITH_PADDING * ratio);
                    }
                }
            }
        }

        self.size = size;
    }

    /// 1つのノードの位置を計算する.
    /// ノードの位置は現在のノード、親ノードの位置、隣り合わせの兄弟ノードによって決定する
    ///
    /// * `node` - 計算対象のノード
    /// * `parent_point` - 親ノードの位置
    /// * `previous_sibling_kind` - 自分より前の兄弟ノードの種類
    /// * `previous_sibling_point` - 自分より前の兄弟ノードの位置
    pub fn compute_position(
        &mut self,
        parent_point: LayoutPoint,
        previous_sibling_kind: LayoutObjectKind,
        previous_sibling_point: Option<LayoutPoint>,
        previous_sibling_size: Option<LayoutSize>,
    ) {
        let mut point = LayoutPoint::new(0, 0);

        // もし自分自身がブロック要素、または、兄弟ノードがブロック要素の場合、このノードは新しい行から描画される、よってウインドウの下方向に向かって位置を調整する
        match (self.kind(), previous_sibling_kind) {
            // もしブロック要素が兄弟ノードの場合、Y 軸方向に進む
            (LayoutObjectKind::Block, _) | (_, LayoutObjectKind::Block) => {
                if let (Some(size), Some(pos)) = (previous_sibling_size, previous_sibling_point) {
                    // もし兄弟ノードが存在する場合、兄弟ノードの Y 位置と高さを足し合わせたものが次の位置になる
                    point.set_y(pos.y() + size.height());
                } else {
                    // もし兄弟ノードが存在しない場合、親ノードの Y 位置が次の位置になる
                    point.set_y(parent_point.y());
                }
                // 新しい行から始まるため、X 軸方向は常に親ノードの X 位置になる
                point.set_x(parent_point.x());
            }
            // もし自分自身と兄弟ノードがともにインライン要素の場合、同じ行に続けて配置されるためウインドウの右方向に向かって位置を調整する。
            // もしインライン要素が並ぶ場合、X 軸方向に進む
            (LayoutObjectKind::Inline, LayoutObjectKind::Inline) => {
                if let (Some(size), Some(pos)) = (previous_sibling_size, previous_sibling_point) {
                    // 兄弟ノードが存在する場合、兄弟ノードの X 位置と横幅を足したものが次の位置になる
                    point.set_x(pos.x() + size.width());
                    // インライン要素は兄弟ノードと同じ行に並ぶため、兄弟ノードのY位置が自身のY位置になる
                    point.set_y(pos.y());
                } else {
                    // もし兄弟ノードが存在しない場合、親ノードの X 座標と Y 座標が次の位置になる
                    point.set_x(parent_point.x());
                    point.set_y(parent_point.y());
                }
            }
            _ => {
                // それ以外の場合、つまりテキストノードのときは親ノードの位置と同じ位置に描画する。
                point.set_x(parent_point.x());
                point.set_y(parent_point.y());
            }
        }

        self.point = point;
    }

    pub fn paint(&mut self) -> Vec<DisplayItem> {
        if self.style.display() == DisplayType::DisplayNone {
            return vec![];
        }

        match self.kind() {
            LayoutObjectKind::Block => {
                if let NodeKind::Element(_e) = self.node_kind() {
                    return vec![DisplayItem::Rect {
                        style: self.style(),
                        layout_point: self.point(),
                        layout_size: self.size(),
                    }];
                }
            }
            LayoutObjectKind::Inline => {
                // 本書のブラウザでは、描画するインライン要素はない。
                // <img> タグなどをサポートした場合はこのアームの中で処理をする
            }
            LayoutObjectKind::Text => {
                if let NodeKind::Text(t) = self.node_kind() {
                    let mut v = vec![];

                    let ratio = match self.style.font_size() {
                        FontSize::Medium => 1,
                        FontSize::XLarge => 2,
                        FontSize::XXLarge => 3,
                    };

                    let plain_text = t
                        .replace("\n", " ")
                        .split(' ')
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>()
                        .join(" ");
                    let lines = split_text(plain_text, CHAR_WIDTH * ratio);
                    let mut i = 0;
                    for line in lines {
                        let item = DisplayItem::Text {
                            text: line,
                            style: self.style(),
                            layout_point: LayoutPoint::new(
                                self.point().x(),
                                self.point().y() + i * CHAR_HEIGHT_WITH_PADDING * i,
                            ),
                        };
                        v.push(item);
                        i += 1;
                    }

                    return v;
                }
            }
        }

        vec![]
    }
}

pub fn create_layout_object(
    node: &Option<Rc<RefCell<Node>>>,
    parent_obj: &Option<Rc<RefCell<LayoutObject>>>,
    cssom: &StyleSheet,
) -> Option<Rc<RefCell<LayoutObject>>> {
    if let Some(n) = node {
        // LayoutObject を作成する
        let layout_object = Rc::new(RefCell::new(LayoutObject::new(n.clone(), parent_obj)));

        // CSS のルールをセレクタで選択されたノードに適用する
        for rule in &cssom.rules {
            if layout_object.borrow().is_node_selected(&rule.selector) {
                layout_object
                    .borrow_mut()
                    .cascading_style(rule.declarations.clone());
            }
        }

        // CSS でスタイルが指定されていない場合、デフォルトの値または親のノードから継承した値を使用する
        let parent_style = if let Some(parent) = parent_obj {
            Some(parent.borrow().style())
        } else {
            None
        };

        layout_object.borrow_mut().defaulting_style(n, parent_style);

        // display プロパティが none の場合、ノードを作成しない
        if layout_object.borrow().style().display() == DisplayType::DisplayNone {
            return None;
        }

        // display プロパティの最終的な値を使用してノードの種類を決定する
        layout_object.borrow_mut().update_kind();
        return Some(layout_object);
    }
    None
}

impl PartialEq for LayoutObject {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

use crate::constants::WINDOW_PADDING;
use crate::constants::WINDOW_WIDTH;
use alloc::string::String;

fn find_index_for_line_break(line: String, max_index: usize) -> usize {
    for i in 0..max_index.rev() {
        if line.chars().collect::<Vec<char>>()[i] == ' ' {
            return i;
        }
    }
    max_index
}

fn split_text(line: String, char_width: i64) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    if line.len() as i64 * char_width > (WINDOW_WIDTH + WINDOW_PADDING) {
        let s = line.split_at(find_index_for_line_break(
            line.clone(),
            ((WINDOW_WIDTH + WINDOW_PADDING) / char_width) as usize,
        ));
        result.push(s.0.to_string());
        result.extend(split_text(s.1.trim().to_string(), char_width));
    } else {
        result.push(line);
    }
    result
}
