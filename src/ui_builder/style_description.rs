use bevy::prelude::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct StyleDescription {
    /// Which layout algorithm to use when laying out this node's contents:
    ///   - [`Display::Flex`]: Use the Flexbox layout algorithm
    ///   - [`Display::Grid`]: Use the CSS Grid layout algorithm
    ///   - [`Display::None`]: Hide this node and perform layout as if it does not exist.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    pub display: Option<Display>,

    /// Whether a node should be laid out in-flow with, or independently of its siblings:
    ///  - [`PositionType::Relative`]: Layout this node in-flow with other nodes using the usual (flexbox/grid) layout algorithm.
    ///  - [`PositionType::Absolute`]: Layout this node on top and independently of other nodes.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/position>
    pub position_type: Option<PositionType>,

    /// Whether overflowing content should be displayed or clipped.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>
    pub overflow: Option<Overflow>,

    /// Defines the text direction. For example, English is written LTR (left-to-right) while Arabic is written RTL (right-to-left).
    ///
    /// Note: the corresponding CSS property also affects box layout order, but this isn't yet implemented in Bevy.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/direction>
    pub direction: Option<Direction>,

    /// The horizontal position of the left edge of the node.
    ///  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
    ///  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/left>
    pub left: Option<Val>,

    /// The horizontal position of the right edge of the node.
    ///  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
    ///  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/right>
    pub right: Option<Val>,

    /// The vertical position of the top edge of the node.
    ///  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
    ///  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/top>
    pub top: Option<Val>,

    /// The vertical position of the bottom edge of the node.
    ///  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
    ///  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/bottom>
    pub bottom: Option<Val>,

    /// The ideal width of the node. `width` is used when it is within the bounds defined by `min_width` and `max_width`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/width>
    pub width: Option<Val>,

    /// The ideal height of the node. `height` is used when it is within the bounds defined by `min_height` and `max_height`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/height>
    pub height: Option<Val>,

    /// The minimum width of the node. `min_width` is used if it is greater than `width` and/or `max_width`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/min-width>
    pub min_width: Option<Val>,

    /// The minimum height of the node. `min_height` is used if it is greater than `height` and/or `max_height`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/min-height>
    pub min_height: Option<Val>,

    /// The maximum width of the node. `max_width` is used if it is within the bounds defined by `min_width` and `width`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/max-width>
    pub max_width: Option<Val>,

    /// The maximum height of the node. `max_height` is used if it is within the bounds defined by `min_height` and `height`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/max-height>
    pub max_height: Option<Val>,

    /// The aspect ratio of the node (defined as `width / height`)
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/aspect-ratio>
    pub aspect_ratio: Option<f32>,

    /// Used to control how each individual item is aligned by default within the space they're given.
    /// - For Flexbox containers, sets default cross axis alignment of the child items.
    /// - For CSS Grid containers, controls block (vertical) axis alignment of children of this grid container within their grid areas.
    ///
    /// This value is overridden if [`AlignSelf`] on the child node is set.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items>
    pub align_items: Option<AlignItems>,

    /// Used to control how each individual item is aligned by default within the space they're given.
    /// - For Flexbox containers, this property has no effect. See `justify_content` for main axis alignment of flex items.
    /// - For CSS Grid containers, sets default inline (horizontal) axis alignment of child items within their grid areas.
    ///
    /// This value is overridden if [`JustifySelf`] on the child node is set.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items>
    pub justify_items: Option<JustifyItems>,

    /// Used to control how the specified item is aligned within the space it's given.
    /// - For Flexbox items, controls cross axis alignment of the item.
    /// - For CSS Grid items, controls block (vertical) axis alignment of a grid item within its grid area.
    ///
    /// If set to `Auto`, alignment is inherited from the value of [`AlignItems`] set on the parent node.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self>
    pub align_self: Option<AlignSelf>,

    /// Used to control how the specified item is aligned within the space it's given.
    /// - For Flexbox items, this property has no effect. See `justify_content` for main axis alignment of flex items.
    /// - For CSS Grid items, controls inline (horizontal) axis alignment of a grid item within its grid area.
    ///
    /// If set to `Auto`, alignment is inherited from the value of [`JustifyItems`] set on the parent node.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self>
    pub justify_self: Option<JustifySelf>,

    /// Used to control how items are distributed.
    /// - For Flexbox containers, controls alignment of lines if `flex_wrap` is set to [`FlexWrap::Wrap`] and there are multiple lines of items.
    /// - For CSS Grid containers, controls alignment of grid rows.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-content>
    pub align_content: Option<AlignContent>,

    /// Used to control how items are distributed.
    /// - For Flexbox containers, controls alignment of items in the main axis.
    /// - For CSS Grid containers, controls alignment of grid columns.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
    pub justify_content: Option<JustifyContent>,

    /// The amount of space around a node outside its border.
    ///
    /// If a percentage value is used, the percentage is calculated based on the width of the parent node.
    ///
    /// # Example
    /// ```
    /// # use bevy_ui::{Style, UiRect, Val};
    /// let style = Style {
    ///     margin: UiRect {
    ///         left: Val::Percent(10.),
    ///         right: Val::Percent(10.),
    ///         top: Val::Percent(15.),
    ///         bottom: Val::Percent(15.)
    ///     },
    ///     ..Default::default()
    /// };
    /// ```
    /// A node with this style and a parent with dimensions of 100px by 300px will have calculated margins of 10px on both left and right edges, and 15px on both top and bottom edges.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin>
    pub margin: Option<UiRect>,

    /// The amount of space between the edges of a node and its contents.
    ///
    /// If a percentage value is used, the percentage is calculated based on the width of the parent node.
    ///
    /// # Example
    /// ```
    /// # use bevy_ui::{Style, UiRect, Val};
    /// let style = Style {
    ///     padding: UiRect {
    ///         left: Val::Percent(1.),
    ///         right: Val::Percent(2.),
    ///         top: Val::Percent(3.),
    ///         bottom: Val::Percent(4.)
    ///     },
    ///     ..Default::default()
    /// };
    /// ```
    /// A node with this style and a parent with dimensions of 300px by 100px will have calculated padding of 3px on the left, 6px on the right, 9px on the top and 12px on the bottom.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding>
    pub padding: Option<UiRect>,

    /// The amount of space between the margins of a node and its padding.
    ///
    /// If a percentage value is used, the percentage is calculated based on the width of the parent node.
    ///
    /// The size of the node will be expanded if there are constraints that prevent the layout algorithm from placing the border within the existing node boundary.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-width>
    pub border: Option<UiRect>,

    /// Whether a Flexbox container should be a row or a column. This property has no effect on Grid nodes.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction>
    pub flex_direction: Option<FlexDirection>,

    /// Whether a Flexbox container should wrap its contents onto multiple lines if they overflow. This property has no effect on Grid nodes.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-wrap>
    pub flex_wrap: Option<FlexWrap>,

    /// Defines how much a flexbox item should grow if there's space available. Defaults to 0 (don't grow at all).
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-grow>
    pub flex_grow: Option<f32>,

    /// Defines how much a flexbox item should shrink if there's not enough space available. Defaults to 1.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-shrink>
    pub flex_shrink: Option<f32>,

    /// The initial length of a flexbox in the main axis, before flex growing/shrinking properties are applied.
    ///
    /// `flex_basis` overrides `size` on the main axis if both are set, but it obeys the bounds defined by `min_size` and `max_size`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-basis>
    pub flex_basis: Option<Val>,

    /// The size of the gutters between items in a vertical flexbox layout or between rows in a grid layout.
    ///
    /// Note: Values of `Val::Auto` are not valid and are treated as zero.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/row-gap>
    pub row_gap: Option<Val>,

    /// The size of the gutters between items in a horizontal flexbox layout or between column in a grid layout.
    ///
    /// Note: Values of `Val::Auto` are not valid and are treated as zero.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/column-gap>
    pub column_gap: Option<Val>,

    /// Controls whether automatically placed grid items are placed row-wise or column-wise as well as whether the sparse or dense packing algorithm is used.
    /// Only affects Grid layouts.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow>
    pub grid_auto_flow: Option<GridAutoFlow>,

    /// Defines the number of rows a grid has and the sizes of those rows. If grid items are given explicit placements then more rows may
    /// be implicitly generated by items that are placed out of bounds. The sizes of those rows are controlled by `grid_auto_rows` property.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-rows>
    pub grid_template_rows: Option<Vec<RepeatedGridTrack>>,

    /// Defines the number of columns a grid has and the sizes of those columns. If grid items are given explicit placements then more columns may
    /// be implicitly generated by items that are placed out of bounds. The sizes of those columns are controlled by `grid_auto_columns` property.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns>
    pub grid_template_columns: Option<Vec<RepeatedGridTrack>>,

    /// Defines the size of implicitly created rows. Rows are created implicitly when grid items are given explicit placements that are out of bounds
    /// of the rows explicitly created using `grid_template_rows`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-rows>
    pub grid_auto_rows: Option<Vec<GridTrack>>,
    /// Defines the size of implicitly created columns. Columns are created implicitly when grid items are given explicit placements that are out of bounds
    /// of the columns explicitly created using `grid_template_columns`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-columns>
    pub grid_auto_columns: Option<Vec<GridTrack>>,

    /// The row in which a grid item starts and how many rows it spans.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row>
    pub grid_row: Option<GridPlacement>,

    /// The column in which a grid item starts and how many columns it spans.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column>
    pub grid_column: Option<GridPlacement>,
}

impl StyleDescription {
    pub(super) fn apply(self, other: &StyleDescription) -> Self {
        Self {
            // Jesus fucking christ there's *gotta* be a better way to write this... T-T
            display: do_override(self.display, other.display),
            position_type: do_override(self.position_type, other.position_type),
            overflow: do_override(self.overflow, other.overflow),
            direction: do_override(self.direction, other.direction),
            left: do_override(self.left, other.left),
            right: do_override(self.right, other.right),
            top: do_override(self.top, other.top),
            bottom: do_override(self.bottom, other.bottom),
            width: do_override(self.width, other.width),
            height: do_override(self.height, other.height),
            min_width: do_override(self.min_width, other.min_width),
            min_height: do_override(self.min_height, other.min_height),
            max_width: do_override(self.max_width, other.max_width),
            max_height: do_override(self.max_height, other.max_height),
            aspect_ratio: do_override(self.aspect_ratio, other.aspect_ratio),
            align_items: do_override(self.align_items, other.align_items),
            justify_items: do_override(self.justify_items, other.justify_items),
            align_self: do_override(self.align_self, other.align_self),
            justify_self: do_override(self.justify_self, other.justify_self),
            align_content: do_override(self.align_content, other.align_content),
            justify_content: do_override(self.justify_content, other.justify_content),
            margin: do_override(self.margin, other.margin),
            padding: do_override(self.padding, other.padding),
            border: do_override(self.border, other.border),
            flex_direction: do_override(self.flex_direction, other.flex_direction),
            flex_wrap: do_override(self.flex_wrap, other.flex_wrap),
            flex_grow: do_override(self.flex_grow, other.flex_grow),
            flex_shrink: do_override(self.flex_shrink, other.flex_shrink),
            flex_basis: do_override(self.flex_basis, other.flex_basis),
            row_gap: do_override(self.row_gap, other.row_gap),
            column_gap: do_override(self.column_gap, other.column_gap),
            grid_auto_flow: do_override(self.grid_auto_flow, other.grid_auto_flow),
            grid_template_rows: do_override(
                self.grid_template_rows,
                other.grid_template_rows.clone(),
            ),
            grid_template_columns: do_override(
                self.grid_template_columns,
                other.grid_template_columns.clone(),
            ),
            grid_auto_rows: do_override(self.grid_auto_rows, other.grid_auto_rows.clone()),
            grid_auto_columns: do_override(self.grid_auto_columns, other.grid_auto_columns.clone()),
            grid_row: do_override(self.grid_row, other.grid_row),
            grid_column: do_override(self.grid_column, other.grid_column),
        }
    }
}

fn do_override<T>(base: Option<T>, next: Option<T>) -> Option<T> {
    if next.is_some() {
        next
    } else {
        base
    }
}
