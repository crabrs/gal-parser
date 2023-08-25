// It is a tree-like structure that is composed of nodes, each of which represents a language element like a variable, operator, or keyword.

// AST Node Definiation Thumb rules:
// - A simplified version of parse tree
//  * Collapse disposible rule
//  * Extract common factors
//  * Each branch node at least have two nodes at least one node. !!! No, unary op is bad case, no !!! no unary op has operator and operand, has two nodes
// - Abstract from the details of the syntax of the language
// - Allow for more efficient processing, more concise and easier to read.
//  * Each enum construction differentiate variants as much as possible

// struct with meta field is a ast node
// the kind field shows the ast node can be different kind.
// the number of meta fields in the path from root to leaf struct is
// the height of ast tree.
//
// if you want to collapse a rule with only one child, choose the better name for that.
// composite query primary: linear query statement, choose linear query statement rather than
// compoiste query primary.
//
// each enum variant must be have obviously different between each other.
//

use std::path::PathBuf;

pub struct Program {
    pub activity: Activity,
    pub close_session: bool,
}

pub enum Activity {
    Session(SessionActivity),
    Transaction(TransactionActivity),
}

pub struct SessionActivity {
    pub commands: Vec<SessionCommand>,
}

pub enum SessionCommand {
    Set(SessionSetCommand),
    Reset,
}

pub enum SessionSetCommand {
    Schema(SessionSetSchema),
    Graph(SessionSetGraph),
    TimeZone(SessionSetTimeZone),
    Parameter(SessionSetParameter),
}

pub struct SessionSetSchema {
    pub schema: SchemaReference,
}

pub struct SessionSetGraph {
    pub graph: GraphExpression,
}

pub struct SessionSetTimeZone {
    pub time_zone: Box<Expr>,
}

pub struct SessionSetParameter {
    pub if_not_exists: bool,
    pub variable_definition: BindingVariableDefinition,
}

pub enum TransactionActivity {
    StartTxn(Option<(ProcedureSpecification, Option<EndTransactionCommand>)>),
    Procedure((ProcedureSpecification, Option<EndTransactionCommand>)),
    EndTxn(EndTransactionCommand),
}

pub enum EndTransactionCommand {
    Rollback,
    Commit,
}

// collapse procedure specification
pub struct ProcedureSpecification {
    pub at_schema: Option<SchemaReference>,
    pub binding_variables: Vec<BindingVariableDefinition>,
    pub statements: (Statement, Vec<(Option<YieldClause>, Statement)>),
}

pub struct BindingVariableDefinition {
    pub ty: Ty,
    pub value: Box<Expr>,
}

pub enum GraphExpression {
    CurrentGraph,
    Reference(GraphReference),
    // collapse:
    //  <nested graph query specification>
    //  <object expression primary>
    //  <object name or binding variable>
    // check semantic in analyze-time
    Expr(Box<Expr>),
}

pub enum BindingTableExpression {
    Reference(BindingTableReference),
    // collapse:
    //  <nested binding table query specification>
    //  <object expression primary>
    //  <object name or binding variable>
    // check semantic in analyze-time
    Expr(Box<Expr>),
}

pub enum Statement {
    LinearCatalogMod(Vec<SimpleCatalogModStatement>),
    LinearDataMod(LinearDataModifyingStatement),
    CompositeQuery(CompositeQueryStatement),
}

pub enum SimpleCatalogModStatement {
    // primitive catalog-modifying procedure statement
    CreateSchema {
        if_not_exists: bool,
        path: PathBuf,
    },
    DropSchema {
        if_exists: bool,
        path: PathBuf,
    },
    CreateGraph {
        // None: no [IF NOT EXISTS] | OR REPLACE
        mode: Option<CreateMode>,
        path: PathBuf,
        ty: CreateGraphAnnotatedType,
        source: Option<GraphExpression>,
    },
    DropGraph {
        if_exists: bool,
        path: PathBuf,
    },
    CreateGraphType {
        // None: no [IF NOT EXISTS] | OR REPLACE
        mode: Option<CreateMode>,
        path: PathBuf,
        souce: CreateGraphTypeSource,
    },
    DropGraphType {
        if_exists: bool,
        path: PathBuf,
    },
    // call catalog-modifying procedure statement
    CallProcedure(CallProcedureStatement),
}

pub enum CreateMode {
    CreateIfNotExists,
    Replace,
}

pub enum CreateGraphAnnotatedType {
    Open,
    Like(GraphExpression),
    Reference(GraphTypeReference),
    Specification(GraphTypeSpecification),
}

pub enum CreateGraphTypeSource {
    // not support external object reference for now
    CopyOf(GraphTypeReference),
    Like(GraphExpression),
    Specification(GraphTypeSpecification),
}

pub enum LinearDataModifyingStatement {
    FocusedLinearDataModifying {
        use_graph: GraphExpression,
        simple_data_accessing_statements: Vec<SimpleDataAccessingStatement>,
        primitive_result_statement: Option<PrimitiveResultStatement>,
    },
    FocusedProcedure {
        use_graph: GraphExpression,
        specification: Box<ProcedureSpecification>,
    },
    AmbientLinearDataModifying {
        simple_data_accessing_statements: Vec<SimpleDataAccessingStatement>,
        primitive_result_statement: Option<PrimitiveResultStatement>,
    },
    AmbientLinearProcedure {
        specification: Box<ProcedureSpecification>,
    },
}

pub enum SimpleDataAccessingStatement {
    Match(MatchStatement),
    Let(LetStatement),
    For(ForStatement),
    Filter(FilterStatement),
    OrderByAndPage(OrderByAndPageStatement),
    Insert(InsertStatement),
    Set(SetStatement),
    Remove(RemoveStatement),
    Delete(DeleteStatement),
    CallProcedure(CallProcedureStatement),
}

pub struct InsertStatement {
    pub graph_pattern: InsertGraphPattern,
}

pub struct SetStatement {
    pub item_list: Vec<SetItem>,
}

pub enum SetItem {
    // set graph element properties
    Properties {
        binding_variable: Ident,
        properties: Vec<(Ident, Box<Expr>)>,
    },
    // set graph element label name
    Label(Ident),
}

pub struct RemoveStatement {
    pub item_list: Vec<RemoveItem>,
}

pub enum RemoveItem {
    Property {
        binding_variable: Ident,
        name: Ident,
    },
    Label(Ident),
}

pub struct DeleteStatement {
    pub mode: DeleteMode,
    pub item_list: Vec<DeleteItem>,
}

pub enum DeleteMode {
    Detach,
    NoDetach,
}

pub type DeleteItem = Box<Expr>;

pub struct CompositeQueryStatement {
    pub queries: (
        LinearQueryStatement,
        Vec<(QueryConjunction, LinearQueryStatement)>,
    ),
}

pub enum QueryConjunction {
    Union(SetQuantifier),
    Except(SetQuantifier),
    Intersect(SetQuantifier),
    Otherwise(SetQuantifier),
}

pub enum LinearQueryStatement {
    FocusedLinearQuery {
        use_graph: GraphExpression,
        simple_query_statements: Vec<SimpleQueryStatement>,
        primitive_result_statement: PrimitiveResultStatement,
    },
    FocusedResult {
        use_graph: GraphExpression,
        prmitive_result_statement: PrimitiveResultStatement,
    },
    FocusedProcedure {
        use_graph: GraphExpression,
        specification: Box<ProcedureSpecification>,
    },
    // Not supported for now
    // Select(SelectStatement),
    AmbientLinearQuery {
        simple_query_statements: Vec<SimpleQueryStatement>,
        primitive_result_statement: PrimitiveResultStatement,
    },
    AmbientProcedure {
        specification: Box<ProcedureSpecification>,
    },
}

pub enum SimpleQueryStatement {
    Match(MatchStatement),
    Let(LetStatement),
    For(ForStatement),
    Filter(FilterStatement),
    OrderByAndPageStatement(OrderByAndPageStatement),
    CallProcedure(CallProcedureStatement),
}

// keyword as a node

pub enum MatchStatement {
    Simple(GraphPatternBindingTable),
    // <simple match statement> | { <simple match statement> | <optional match statement> }...
    Optional(Vec<MatchStatement>),
}

pub struct LetStatement {
    pub variable_definitions: Vec<VariableDefinition<Ty, Expr>>,
}

pub struct ForStatement {
    pub item_alias: Ident,
    pub list_value_expression: Box<Expr>,
    pub ordinality_or_offset: OrdinalityOfOffset,
}

pub enum OrdinalityOfOffset {
    Ordinality(Ident),
    Offset(Ident),
}

pub struct FilterStatement {
    pub condition: Box<Expr>,
}

pub struct OrderByAndPageStatement {
    pub kind: OrderByAndPageStatementKind,
}

pub enum OrderByAndPageStatementKind {
    OrderByOffsetLimit {
        order_by: OrderByClause,
        offset: Option<OffsetClause>,
        limit: Option<LimitClause>,
    },
    OffsetLimit {
        offset: OffsetClause,
        limit: Option<LimitClause>,
    },
    Limit(LimitClause),
}

pub enum PrimitiveResultStatement {
    Result {
        return_statement: ReturnStatement,
        order_by_and_page_statement: Option<OrderByAndPageStatement>,
    },
    Finish,
}

pub struct ReturnStatement {
    // NO BINDINGS = Some(All),
    pub set_quantifier: SetQuantifier,
    pub return_item_list: ReturnItemList,
    pub group_by_clause: Option<GroupByClause>,
}

pub enum ReturnItemList {
    Wildcard,
    // expr AS n
    List(Vec<(Box<Expr>, Option<Ident>)>),
}

// NOT supported for now
// pub struct SelectStatement {
//     pub meta: NodeMeta,
// }

pub type SelectItemList = ReturnItemList;

pub struct OrderByClause {
    pub sort_specifications: Vec<(Box<Expr>, OrderingSpecification, Option<NullOrdering>)>,
}

pub enum OrderingSpecification {
    ASC,
    DESC,
}

pub enum NullOrdering {
    NullsFirst,
    NullsLast,
}

pub struct OffsetClause {
    pub offset_count: usize,
}

pub struct LimitClause {
    pub limit_count: usize,
}

pub struct CallProcedureStatement {
    pub optional: bool,
    pub procedure_call: ProcedureCall,
}

pub enum ProcedureCall {
    Inline {
        variable_scope: Vec<Ident>,
        procedure: ProcedureSpecification,
    },
    Named {
        reference: ProcedureReference,
        arguments: Vec<Expr>,
        yield_clause: Option<YieldClause>,
    },
}

pub struct YieldClause {
    pub item_list: Vec<(Ident, Option<Ident>)>,
}

pub struct GroupByClause {
    pub element_list: Vec<Ident>,
}

pub struct VariableDefinition<T, V> {
    pub name: Ident,
    pub ty: Option<T>,
    pub value: V,
}

pub struct InsertGraphPattern {}

pub enum SetQuantifier {
    Distinct,
    All,
}

pub struct GraphPatternBindingTable {
    pub pattern: GraphPattern,
    pub yield_items: Vec<Ident>,
}

pub struct GraphPattern {
    pub match_mode: MatchMode,
    pub path_pattern_list: Vec<PathPattern>,
    pub keep: PathPatternPrefix,
    pub where_condition: Option<Box<Expr>>,
}

pub enum MatchMode {
    Repeatable,
    Different,
}

pub struct PathPattern {
    pub variable: Option<Ident>,
    pub prefix: Option<PathPatternPrefix>,
    pub path_pattern_expression: Box<PathPatternExpression>,
}

pub enum PathPatternPrefix {
    All {
        mode: PathMode,
    },
    Any {
        number_of_paths: usize,
        mode: PathMode,
    },
    AllShortest {
        mode: PathMode,
    },
    AnyShortest {
        mode: PathMode,
    },
    CountedShortest {
        number_of_paths: usize,
        mode: PathMode,
    },
    CountedShortestGroup {
        number_of_groups: usize,
        mode: PathMode,
    },
}

pub enum PathMode {
    Walk,
    Trail,
    Simple,
    Acyclic,
}

pub struct PathPatternExpression {
    // actually should be <element pattern> | <parenthesized path pattern expression> | <simplified path pattern expression>
    // only support element pattern chain for now
    pub path_terms: Vec<ElementPattern>,
}

pub enum ElementPattern {
    Node {
        filter: ElementPatternFilter,
    },
    Edge {
        kind: EdgePatternKind,
        filter: ElementPatternFilter,
        quantifier: Option<EdgeQuantifier>,
    },
}

pub enum EdgePatternKind {
    // <-[]-
    Left,
    // ~[]~
    Undirected,
    // -[]->
    Right,
    // <~[]-
    LeftOrUndirected,
    // -[]~>
    UndirectedOrRight,
    // <-[]->
    LeftOrRight,
    // -[]-
    Any,
}

pub struct ElementPatternFilter {
    pub variable: Option<Ident>,
    pub is_label: Option<Box<LabelExpression>>,
    pub predicate: Option<ElementPatternPredicate>,
}

pub enum ElementPatternPredicate {
    Where(Box<Expr>),
    Properties(Vec<(Ident, Box<Expr>)>),
}

pub enum EdgeQuantifier {
    Questioned,
    Quantified((Option<usize>, Option<usize>)),
}

pub enum LabelExpression {
    Term(LabelTerm),
    Disjunct((Box<LabelExpression>, LabelTerm)),
}

pub enum LabelTerm {
    Factor(LabelFactor),
    Conjunct((Box<LabelTerm>, LabelFactor)),
}

pub enum LabelFactor {
    Primary(LabelPrimary),
    Neg(LabelPrimary),
}

pub enum LabelPrimary {
    Name(Ident),
    Wildcard,
    Parenthesized(Box<LabelExpression>),
}

pub enum Expr {
    Ident(Ident),
    Lit(Lit),
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    IsFalse(Box<Expr>),
    IsNotFalse(Box<Expr>),
    IsTrue(Box<Expr>),
    IsNotTrue(Box<Expr>),
    IsNull(Box<Expr>),
    IsNotNull(Box<Expr>),
    IsUnknown(Box<Expr>),
    IsNotUnknown(Box<Expr>),
    IsTyped {
        expr: Box<Expr>,
        ty: Ty,
    },
    IsNotTyped {
        expr: Box<Expr>,
        ty: Ty,
    },
    // not supported is normalized for now
    // IsNormalized(Box<Expr>, NormalForm),
    // IsNotNormalized(Box<Expr>, NormalForm),
    IsDirected(Ident),
    IsNotDirected(Ident),
    // !Maker & Director | Actor
    IsLabeled(Ident),
    IsNotLabeled(Ident),
    IsSourceOf {
        node: Ident,
        edge: Ident,
    },
    IsNotSourceOf {
        node: Ident,
        edge: Ident,
    },
    IsDestinationOf {
        node: Ident,
        edge: Ident,
    },
    IsNotDestinationOf {
        node: Ident,
        edge: Ident,
    },
    AllDifferent(Vec<Ident>),
    Same(Vec<Ident>),
    // element variable, property name
    PropertyExists(Ident, Ident),
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    CharLength(Box<Expr>),
    ByteLength(Box<Expr>),
    PathLength(Box<Expr>),
    Abs(Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    // not suppport trigonometric function for now
    // not suppport logarithm function for now
    Sqrt(Box<Expr>),
    Floor(Box<Expr>),
    Ceil(Box<Expr>),
    Left(Box<Expr>, Box<Expr>),
    Right(Box<Expr>, Box<Expr>),
    Upper(Box<Expr>),
    Lower(Box<Expr>),
    StringTrim {
        trim_spec: Option<TrimSpec>,
        trim_string: Option<Box<Expr>>,
        trim_source: Box<Expr>,
    },
    Date(Option<Box<Expr>>),
    ZonedTime(Option<Box<Expr>>),
    LocalTime(Option<Box<Expr>>),
    ZonedDateTime(Option<Box<Expr>>),
    LocalDateTime(Option<Box<Expr>>),
    DurationBetween(Box<Expr>, Box<Expr>),
    Duration(Option<Box<Expr>>),
    List {
        is_grouped: bool,
        elements: Vec<Expr>,
    },
    ListTrim(Box<Expr>, Box<Expr>),
    Elements(Box<Expr>),
    Record(Vec<(Ident, Box<Expr>)>),
    Path(Vec<Expr>),
    PropRef(Box<Expr>, Ident),
    Query(ProcedureSpecification),
    NullIf(Box<Expr>, Box<Expr>),
    Coalesce(Vec<Expr>),
    SimpleCase {
        case_operand: Box<Expr>,
        when_clauses: Vec<(Vec<Expr>, Box<Expr>)>,
        else_clause: Option<Box<Expr>>,
    },
    SearchedCase {
        when_clauses: Vec<Expr>,
        else_clause: Option<Box<Expr>>,
    },
    Cast {
        operand: Box<Expr>,
        target: Ty,
    },
    ElementID(Ident),
    // not support let value expression for now
}

pub enum BinOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Concat,
    Gt,
    Lt,
    GtEq,
    LtEq,
    Eq,
    NotEq,
    And,
    Or,
    Xor,
}

pub enum TrimSpec {
    Leading,
    Trailing,
    Both,
}

pub enum UnaryOp {
    Plus,
    Minus,
    Not,
}

pub enum Lit {
    Integer(String),
    Float(String),
    // STRING_LITERAL
    CharString(String),
    // like rust: b"\x3f\x32\x12";
    // https://doc.rust-lang.org/beta/reference/tokens.html#byte-and-byte-string-literals
    ByteString(Vec<u8>),
    Boolean(bool),
    DateTime(String),
    Date(String),
    Time(String),
    Duration(String),
    Null,
}

pub enum SchemaReference {
    Home,
    Current,
    Path(PathBuf),
    Parameter(Ident),
}

pub struct GraphReference {
    pub kind: GraphReferenceKind,
}

pub enum GraphReferenceKind {
    Home,
    Name(Ident),
    Path(PathBuf),
    Parameter(Ident),
}

pub enum BindingTableReference {
    Path(PathBuf),
    Name(Ident),
    Parameter(Ident),
}

pub enum GraphTypeReference {
    Path(PathBuf),
    Parameter(Ident),
}

pub enum ProcedureReference {
    Path(PathBuf),
    Parameter(Ident),
}

// nested graph type specification
pub struct GraphTypeSpecification {
    pub element_type_definitions: Vec<ElementTypeDefinition>,
}

pub enum ElementTypeDefinition {
    Node(NodeTypeDefinition),
    Edge(EdgeTypeDefinition),
}

pub struct NodeTypeDefinition {
    pub ty_name: Option<Ident>,
    pub ty_filter: Option<ElementTypeFilter>,
}

pub struct EdgeTypeDefinition {
    pub kind: EdgeKind,
    pub ty_name: Option<Ident>,
    pub ty_filter: Option<ElementTypeFilter>,
    pub endpoint_definition: Option<EndpointDefinition>,
}

pub enum EndpointDefinition {
    // 0 -> 1
    PointingLeft(Ident, Ident),
    // 0 <- 1
    PointingRight(Ident, Ident),
    // 0 ~ 1
    Undirected(Ident, Ident),
}

pub enum EdgeKind {
    Directed,
    Undirected,
}

pub struct ElementTypeFilter {
    pub label_set: Vec<Ident>,
    pub property_ty_set: Vec<PropertyTypeDefinition>,
}

pub struct PropertyTypeDefinition {
    pub name: Ident,
    pub ty: Ty,
}

pub enum Ty {
    // bool [not null]
    Boolean {
        not_null: bool,
    },
    String {
        max_length: Option<usize>,
        not_null: bool,
    },
    Bytes {
        min_max_length: Option<(Option<usize>, usize)>,
        not_null: bool,
    },
    Integer {
        not_null: bool,
    },
    Float {
        not_null: bool,
    },
    DateTime {
        not_null: bool,
    },
    LocalDateTime {
        not_null: bool,
    },
    Date {
        not_null: bool,
    },
    Time {
        not_null: bool,
    },
    LocalTime {
        not_null: bool,
    },
    GraphReference(GraphReferenceValueType),
    BindingTable {
        fields: Vec<(Ident, Box<Ty>)>,
        not_null: bool,
    },
    NodeReference(NodeReferenceValueType),
    EdgeReference(EdgeReferenceValueType),
    List {
        grouped: bool,
        value_ty: Box<Ty>,
        max_length: Option<usize>,
        not_null: bool,
    },
    Record(RecordType),
    Path {
        not_null: bool,
    },
}

pub enum RecordType {
    Any {
        not_null: bool,
    },
    FieldSpec {
        fields: Vec<(Ident, Box<Ty>)>,
        not_null: bool,
    },
}

pub enum GraphReferenceValueType {
    Open {
        not_null: bool,
    },
    Closed {
        graph_ty_specification: GraphTypeSpecification,
        not_null: bool,
    },
}

pub enum NodeReferenceValueType {
    Open {
        not_null: bool,
    },
    Closed {
        node_ty_definition: NodeTypeDefinition,
        not_null: bool,
    },
}

pub enum EdgeReferenceValueType {
    Open {
        not_null: bool,
    },
    Closed {
        edge_ty_definition: EdgeTypeDefinition,
        not_null: bool,
    },
}

pub struct Ident {
    name: String,
}
