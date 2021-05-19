// Example code that deserializes and serializes the model.
//"This file is a JSON schema of files created by LDtk level editor (https://ldtk.io)."
//title	"LDtk 0.9.3 JSON schema"
//$schema	"https://json-schema.org/draft-07/schema#"
//$ref	"#/LdtkJsonRoot"
//version	"0.9.3

use serde::*;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub description: String,
    pub title: String,
    #[serde(rename = "$schema")]
    pub schema: String,
    #[serde(rename = "$ref")]
    pub project_ref: String,
    pub version: String,
    #[serde(rename = "LdtkJsonRoot")]
    pub ldtk_json_root: LdtkJsonRoot,
    #[serde(rename = "otherTypes")]
    pub other_types: OtherTypes,
}

#[derive(Serialize, Deserialize)]
pub struct LdtkJsonRoot {
    pub description: String,
    pub title: String,
    pub required: Vec<String>,
    pub properties: LdtkJsonRootProperties,
    #[serde(rename = "type")]
    pub ldtk_json_root_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct LdtkJsonRootProperties {
    #[serde(rename = "backupLimit")]
    pub backup_limit: BackupLimit,
    #[serde(rename = "backupOnSave")]
    pub backup_on_save: BackupLimit,
    #[serde(rename = "worldGridWidth")]
    pub world_grid_width: BackupLimit,
    #[serde(rename = "defaultLevelBgColor")]
    pub default_level_bg_color: BackupLimit,
    #[serde(rename = "bgColor")]
    pub bg_color: BackupLimit,
    #[serde(rename = "nextUid")]
    pub next_uid: BackupLimit,
    #[serde(rename = "imageExportMode")]
    pub image_export_mode: ImageExportMode,
    #[serde(rename = "defaultPivotY")]
    pub default_pivot_y: BackupLimit,
    #[serde(rename = "worldGridHeight")]
    pub world_grid_height: BackupLimit,
    #[serde(rename = "defaultGridSize")]
    pub default_grid_size: BackupLimit,
    #[serde(rename = "worldLayout")]
    pub world_layout: ImageExportMode,
    pub flags: Flags,
    #[serde(rename = "levelNamePattern")]
    pub level_name_pattern: BackupLimit,
    #[serde(rename = "exportPng")]
    pub export_png: BackupLimit,
    #[serde(rename = "defaultLevelWidth")]
    pub default_level_width: BackupLimit,
    #[serde(rename = "pngFilePattern")]
    pub png_file_pattern: BackupLimit,
    #[serde(rename = "exportTiled")]
    pub export_tiled: BackupLimit,
    pub defs: Defs,
    pub levels: Levels,
    #[serde(rename = "jsonVersion")]
    pub json_version: BackupLimit,
    #[serde(rename = "defaultPivotX")]
    pub default_pivot_x: BackupLimit,
    #[serde(rename = "defaultLevelHeight")]
    pub default_level_height: BackupLimit,
    #[serde(rename = "externalLevels")]
    pub external_levels: BackupLimit,
    #[serde(rename = "minifyJson")]
    pub minify_json: BackupLimit,
}

#[derive(Serialize, Deserialize)]
pub struct BackupLimit {
    pub description: String,
    #[serde(rename = "type")]
    pub backup_limit_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct Defs {
    pub description: String,
    #[serde(rename = "$ref")]
    pub defs_ref: String,
}

#[derive(Serialize, Deserialize)]
pub struct Flags {
    pub description: String,
    pub items: FlagsItems,
    #[serde(rename = "type")]
    pub flags_type: Vec<FlagsType>,
}

#[derive(Serialize, Deserialize)]
pub struct FlagsItems {
    #[serde(rename = "enum")]
    pub items_enum: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ImageExportMode {
    pub description: String,
    #[serde(rename = "enum")]
    pub image_export_mode_enum: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Levels {
    pub description: String,
    pub items: LevelsItems,
    #[serde(rename = "type")]
    pub levels_type: Vec<FlagsType>,
}

#[derive(Serialize, Deserialize)]
pub struct LevelsItems {
    #[serde(rename = "$ref")]
    pub items_ref: String,
}

#[derive(Serialize, Deserialize)]
pub struct OtherTypes {
    #[serde(rename = "FieldInstance")]
    pub field_instance: FieldInstance,
    #[serde(rename = "EntityInstance")]
    pub entity_instance: EntityInstance,
    #[serde(rename = "Definitions")]
    pub definitions: Definitions,
    #[serde(rename = "AutoRuleDef")]
    pub auto_rule_def: AutoRuleDef,
    #[serde(rename = "FieldDef")]
    pub field_def: FieldDef,
    #[serde(rename = "EntityDef")]
    pub entity_def: EntityDef,
    #[serde(rename = "AutoLayerRuleGroup")]
    pub auto_layer_rule_group: AutoLayerRuleGroup,
    #[serde(rename = "EntityInstanceTile")]
    pub entity_instance_tile: EntityInstanceTile,
    #[serde(rename = "IntGridValueInstance")]
    pub int_grid_value_instance: IntGridValueInstance,
    #[serde(rename = "NeighbourLevel")]
    pub neighbour_level: NeighbourLevel,
    #[serde(rename = "LayerInstance")]
    pub layer_instance: LayerInstance,
    #[serde(rename = "TilesetDef")]
    pub tileset_def: TilesetDef,
    #[serde(rename = "EnumDefValues")]
    pub enum_def_values: EnumDefValues,
    #[serde(rename = "Tile")]
    pub tile: TileClass,
    #[serde(rename = "LayerDef")]
    pub layer_def: LayerDef,
    #[serde(rename = "LevelBgPosInfos")]
    pub level_bg_pos_infos: LevelBgPosInfos,
    #[serde(rename = "Level")]
    pub level: Level,
    #[serde(rename = "EnumDef")]
    pub enum_def: EnumDef,
    #[serde(rename = "IntGridValueDef")]
    pub int_grid_value_def: IntGridValueDef,
}

#[derive(Serialize, Deserialize)]
pub struct AutoLayerRuleGroup {
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: AutoLayerRuleGroupProperties,
    #[serde(rename = "type")]
    pub auto_layer_rule_group_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct AutoLayerRuleGroupProperties {
    pub name: BackupLimit,
    pub collapsed: BackupLimit,
    #[serde(rename = "isOptional")]
    pub is_optional: BackupLimit,
    pub uid: BackupLimit,
    pub active: BackupLimit,
    pub rules: Levels,
}

#[derive(Serialize, Deserialize)]
pub struct AutoRuleDef {
    pub description: String,
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: AutoRuleDefProperties,
    #[serde(rename = "type")]
    pub auto_rule_def_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct AutoRuleDefProperties {
    #[serde(rename = "flipX")]
    pub flip_x: BackupLimit,
    #[serde(rename = "pivotX")]
    pub pivot_x: BackupLimit,
    #[serde(rename = "perlinActive")]
    pub perlin_active: BackupLimit,
    #[serde(rename = "perlinScale")]
    pub perlin_scale: BackupLimit,
    #[serde(rename = "outOfBoundsValue")]
    pub out_of_bounds_value: BackupLimit,
    pub pattern: Pattern,
    pub checker: ImageExportMode,
    #[serde(rename = "perlinOctaves")]
    pub perlin_octaves: BackupLimit,
    #[serde(rename = "tileIds")]
    pub tile_ids: Pattern,
    #[serde(rename = "xModulo")]
    pub x_modulo: BackupLimit,
    pub size: BackupLimit,
    pub chance: BackupLimit,
    #[serde(rename = "breakOnMatch")]
    pub break_on_match: BackupLimit,
    pub uid: BackupLimit,
    #[serde(rename = "perlinSeed")]
    pub perlin_seed: BackupLimit,
    #[serde(rename = "tileMode")]
    pub tile_mode: ImageExportMode,
    #[serde(rename = "flipY")]
    pub flip_y: BackupLimit,
    #[serde(rename = "pivotY")]
    pub pivot_y: BackupLimit,
    #[serde(rename = "yModulo")]
    pub y_modulo: BackupLimit,
    pub active: BackupLimit,
}

#[derive(Serialize, Deserialize)]
pub struct Pattern {
    pub description: String,
    pub items: PatternItems,
    #[serde(rename = "type")]
    pub pattern_type: Vec<FlagsType>,
}

#[derive(Serialize, Deserialize)]
pub struct PatternItems {
    #[serde(rename = "type")]
    pub items_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct Definitions {
    pub description: String,
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: DefinitionsProperties,
    #[serde(rename = "type")]
    pub definitions_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct DefinitionsProperties {
    pub tilesets: Levels,
    pub layers: Levels,
    #[serde(rename = "levelFields")]
    pub level_fields: Levels,
    pub enums: Levels,
    pub entities: Levels,
    #[serde(rename = "externalEnums")]
    pub external_enums: Levels,
}

#[derive(Serialize, Deserialize)]
pub struct EntityDef {
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: EntityDefProperties,
    #[serde(rename = "type")]
    pub entity_def_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityDefProperties {
    #[serde(rename = "tileId")]
    pub tile_id: BackupLimit,
    #[serde(rename = "showName")]
    pub show_name: BackupLimit,
    #[serde(rename = "tilesetId")]
    pub tileset_id: BackupLimit,
    #[serde(rename = "limitScope")]
    pub limit_scope: ImageExportMode,
    #[serde(rename = "pivotX")]
    pub pivot_x: BackupLimit,
    #[serde(rename = "maxCount")]
    pub max_count: BackupLimit,
    pub hollow: BackupLimit,
    #[serde(rename = "keepAspectRatio")]
    pub keep_aspect_ratio: BackupLimit,
    pub color: BackupLimit,
    #[serde(rename = "fieldDefs")]
    pub field_defs: Levels,
    #[serde(rename = "tileRenderMode")]
    pub tile_render_mode: ImageExportMode,
    #[serde(rename = "limitBehavior")]
    pub limit_behavior: ImageExportMode,
    #[serde(rename = "resizableX")]
    pub resizable_x: BackupLimit,
    pub uid: BackupLimit,
    #[serde(rename = "lineOpacity")]
    pub line_opacity: BackupLimit,
    #[serde(rename = "resizableY")]
    pub resizable_y: BackupLimit,
    #[serde(rename = "fillOpacity")]
    pub fill_opacity: BackupLimit,
    pub height: BackupLimit,
    pub identifier: BackupLimit,
    #[serde(rename = "pivotY")]
    pub pivot_y: BackupLimit,
    #[serde(rename = "renderMode")]
    pub render_mode: ImageExportMode,
    pub tags: Pattern,
    pub width: BackupLimit,
}

#[derive(Serialize, Deserialize)]
pub struct EntityInstance {
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: EntityInstanceProperties,
    #[serde(rename = "type")]
    pub entity_instance_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityInstanceProperties {
    #[serde(rename = "defUid")]
    pub def_uid: BackupLimit,
    #[serde(rename = "__identifier")]
    pub identifier: BackupLimit,
    #[serde(rename = "__tile")]
    pub tile: Tile,
    pub px: Pattern,
    #[serde(rename = "__grid")]
    pub grid: Pattern,
    #[serde(rename = "__pivot")]
    pub pivot: Pattern,
    #[serde(rename = "fieldInstances")]
    pub field_instances: Levels,
    pub height: BackupLimit,
    pub width: BackupLimit,
}

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub description: String,
    #[serde(rename = "oneOf")]
    pub one_of: Vec<OneOf>,
}

#[derive(Serialize, Deserialize)]
pub struct OneOf {
    #[serde(rename = "type")]
    pub one_of_type: Option<Vec<BackupLimitType>>,
    #[serde(rename = "$ref")]
    pub one_of_ref: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityInstanceTile {
    pub description: String,
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: EntityInstanceTileProperties,
    #[serde(rename = "type")]
    pub entity_instance_tile_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityInstanceTileProperties {
    #[serde(rename = "srcRect")]
    pub src_rect: Pattern,
    #[serde(rename = "tilesetUid")]
    pub tileset_uid: BackupLimit,
}

#[derive(Serialize, Deserialize)]
pub struct EnumDef {
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: EnumDefProperties,
    #[serde(rename = "type")]
    pub enum_def_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct EnumDefProperties {
    #[serde(rename = "externalFileChecksum")]
    pub external_file_checksum: BackupLimit,
    #[serde(rename = "externalRelPath")]
    pub external_rel_path: BackupLimit,
    pub uid: BackupLimit,
    pub values: Levels,
    #[serde(rename = "iconTilesetUid")]
    pub icon_tileset_uid: BackupLimit,
    pub identifier: BackupLimit,
}

#[derive(Serialize, Deserialize)]
pub struct EnumDefValues {
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: EnumDefValuesProperties,
    #[serde(rename = "type")]
    pub enum_def_values_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct EnumDefValuesProperties {
    #[serde(rename = "tileId")]
    pub tile_id: BackupLimit,
    pub color: BackupLimit,
    pub id: BackupLimit,
    #[serde(rename = "__tileSrcRect")]
    pub tile_src_rect: Pattern,
}

#[derive(Serialize, Deserialize)]
pub struct FieldDef {
    pub description: String,
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: FieldDefProperties,
    #[serde(rename = "type")]
    pub field_def_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct FieldDefProperties {
    #[serde(rename = "acceptFileTypes")]
    pub accept_file_types: Pattern,
    #[serde(rename = "editorAlwaysShow")]
    pub editor_always_show: BackupLimit,
    #[serde(rename = "arrayMinLength")]
    pub array_min_length: BackupLimit,
    pub min: BackupLimit,
    #[serde(rename = "__type")]
    pub properties_type: BackupLimit,
    #[serde(rename = "editorDisplayMode")]
    pub editor_display_mode: ImageExportMode,
    #[serde(rename = "canBeNull")]
    pub can_be_null: BackupLimit,
    pub uid: BackupLimit,
    #[serde(rename = "isArray")]
    pub is_array: BackupLimit,
    #[serde(rename = "editorDisplayPos")]
    pub editor_display_pos: ImageExportMode,
    #[serde(rename = "textLanguageMode")]
    pub text_language_mode: TextLanguageMode,
    pub max: BackupLimit,
    #[serde(rename = "editorCutLongValues")]
    pub editor_cut_long_values: BackupLimit,
    #[serde(rename = "defaultOverride")]
    pub default_override: DefaultOverride,
    pub regex: BackupLimit,
    #[serde(rename = "type")]
    pub purple_type: DefaultOverride,
    pub identifier: BackupLimit,
    #[serde(rename = "arrayMaxLength")]
    pub array_max_length: BackupLimit,
}

#[derive(Serialize, Deserialize)]
pub struct DefaultOverride {
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct TextLanguageMode {
    pub description: String,
    #[serde(rename = "enum")]
    pub text_language_mode_enum: Vec<Option<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct FieldInstance {
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: FieldInstanceProperties,
    #[serde(rename = "type")]
    pub field_instance_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct FieldInstanceProperties {
    #[serde(rename = "__type")]
    pub properties_type: BackupLimit,
    #[serde(rename = "defUid")]
    pub def_uid: BackupLimit,
    #[serde(rename = "__identifier")]
    pub identifier: BackupLimit,
    #[serde(rename = "realEditorValues")]
    pub real_editor_values: RealEditorValues,
    #[serde(rename = "__value")]
    pub value: DefaultOverride,
}

#[derive(Serialize, Deserialize)]
pub struct RealEditorValues {
    pub description: String,
    pub items: RealEditorValuesItems,
    #[serde(rename = "type")]
    pub real_editor_values_type: Vec<FlagsType>,
}

#[derive(Serialize, Deserialize)]
pub struct RealEditorValuesItems {
}

#[derive(Serialize, Deserialize)]
pub struct IntGridValueDef {
    pub description: String,
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: IntGridValueDefProperties,
    #[serde(rename = "type")]
    pub int_grid_value_def_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct IntGridValueDefProperties {
    pub color: BackupLimit,
    pub identifier: BackupLimit,
    pub value: BackupLimit,
}

#[derive(Serialize, Deserialize)]
pub struct IntGridValueInstance {
    pub description: String,
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: IntGridValueInstanceProperties,
    #[serde(rename = "type")]
    pub int_grid_value_instance_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct IntGridValueInstanceProperties {
    pub v: BackupLimit,
    #[serde(rename = "coordId")]
    pub coord_id: BackupLimit,
}

#[derive(Serialize, Deserialize)]
pub struct LayerDef {
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: LayerDefProperties,
    #[serde(rename = "type")]
    pub layer_def_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct LayerDefProperties {
    #[serde(rename = "pxOffsetX")]
    pub px_offset_x: BackupLimit,
    #[serde(rename = "tilePivotX")]
    pub tile_pivot_x: BackupLimit,
    #[serde(rename = "displayOpacity")]
    pub display_opacity: BackupLimit,
    #[serde(rename = "__type")]
    pub properties_type: BackupLimit,
    #[serde(rename = "tilesetDefUid")]
    pub tileset_def_uid: BackupLimit,
    #[serde(rename = "autoSourceLayerDefUid")]
    pub auto_source_layer_def_uid: BackupLimit,
    #[serde(rename = "autoTilesetDefUid")]
    pub auto_tileset_def_uid: BackupLimit,
    pub uid: BackupLimit,
    #[serde(rename = "excludedTags")]
    pub excluded_tags: Pattern,
    #[serde(rename = "intGridValues")]
    pub int_grid_values: Levels,
    #[serde(rename = "autoRuleGroups")]
    pub auto_rule_groups: Levels,
    #[serde(rename = "type")]
    pub purple_type: ImageExportMode,
    pub identifier: BackupLimit,
    #[serde(rename = "requiredTags")]
    pub required_tags: Pattern,
    #[serde(rename = "pxOffsetY")]
    pub px_offset_y: BackupLimit,
    #[serde(rename = "tilePivotY")]
    pub tile_pivot_y: BackupLimit,
    #[serde(rename = "gridSize")]
    pub grid_size: BackupLimit,
}

#[derive(Serialize, Deserialize)]
pub struct LayerInstance {
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: LayerInstanceProperties,
    #[serde(rename = "type")]
    pub layer_instance_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct LayerInstanceProperties {
    #[serde(rename = "__cHei")]
    pub c_hei: BackupLimit,
    #[serde(rename = "pxOffsetX")]
    pub px_offset_x: BackupLimit,
    #[serde(rename = "__tilesetRelPath")]
    pub tileset_rel_path: BackupLimit,
    #[serde(rename = "levelId")]
    pub level_id: BackupLimit,
    #[serde(rename = "__type")]
    pub properties_type: BackupLimit,
    #[serde(rename = "autoLayerTiles")]
    pub auto_layer_tiles: Levels,
    #[serde(rename = "optionalRules")]
    pub optional_rules: Pattern,
    #[serde(rename = "__identifier")]
    pub identifier: BackupLimit,
    #[serde(rename = "__gridSize")]
    pub grid_size: BackupLimit,
    #[serde(rename = "__pxTotalOffsetY")]
    pub px_total_offset_y: BackupLimit,
    #[serde(rename = "intGridCsv")]
    pub int_grid_csv: Pattern,
    #[serde(rename = "overrideTilesetUid")]
    pub override_tileset_uid: BackupLimit,
    pub visible: BackupLimit,
    #[serde(rename = "entityInstances")]
    pub entity_instances: Levels,
    #[serde(rename = "__opacity")]
    pub opacity: BackupLimit,
    pub seed: BackupLimit,
    #[serde(rename = "layerDefUid")]
    pub layer_def_uid: BackupLimit,
    #[serde(rename = "__pxTotalOffsetX")]
    pub px_total_offset_x: BackupLimit,
    #[serde(rename = "__cWid")]
    pub c_wid: BackupLimit,
    #[serde(rename = "pxOffsetY")]
    pub px_offset_y: BackupLimit,
    #[serde(rename = "__tilesetDefUid")]
    pub tileset_def_uid: BackupLimit,
    #[serde(rename = "gridTiles")]
    pub grid_tiles: Levels,
    #[serde(rename = "intGrid")]
    pub int_grid: Levels,
}

#[derive(Serialize, Deserialize)]
pub struct Level {
    pub description: String,
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: LevelProperties,
    #[serde(rename = "type")]
    pub level_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct LevelProperties {
    #[serde(rename = "__neighbours")]
    pub neighbours: Levels,
    #[serde(rename = "__bgColor")]
    pub bg_color: BackupLimit,
    #[serde(rename = "worldX")]
    pub world_x: BackupLimit,
    #[serde(rename = "externalRelPath")]
    pub external_rel_path: BackupLimit,
    #[serde(rename = "useAutoIdentifier")]
    pub use_auto_identifier: BackupLimit,
    #[serde(rename = "bgColor")]
    pub properties_bg_color: BackupLimit,
    #[serde(rename = "bgPos")]
    pub properties_bg_pos: TextLanguageMode,
    #[serde(rename = "pxHei")]
    pub px_hei: BackupLimit,
    #[serde(rename = "worldY")]
    pub world_y: BackupLimit,
    #[serde(rename = "__bgPos")]
    pub bg_pos: Tile,
    pub uid: BackupLimit,
    #[serde(rename = "fieldInstances")]
    pub field_instances: Levels,
    #[serde(rename = "pxWid")]
    pub px_wid: BackupLimit,
    pub identifier: BackupLimit,
    #[serde(rename = "bgPivotY")]
    pub bg_pivot_y: BackupLimit,
    #[serde(rename = "bgPivotX")]
    pub bg_pivot_x: BackupLimit,
    #[serde(rename = "layerInstances")]
    pub layer_instances: Levels,
    #[serde(rename = "bgRelPath")]
    pub bg_rel_path: BackupLimit,
}

#[derive(Serialize, Deserialize)]
pub struct LevelBgPosInfos {
    pub description: String,
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: LevelBgPosInfosProperties,
    #[serde(rename = "type")]
    pub level_bg_pos_infos_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct LevelBgPosInfosProperties {
    #[serde(rename = "cropRect")]
    pub crop_rect: Pattern,
    pub scale: Pattern,
    #[serde(rename = "topLeftPx")]
    pub top_left_px: Pattern,
}

#[derive(Serialize, Deserialize)]
pub struct NeighbourLevel {
    pub description: String,
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: NeighbourLevelProperties,
    #[serde(rename = "type")]
    pub neighbour_level_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct NeighbourLevelProperties {
    #[serde(rename = "levelUid")]
    pub level_uid: BackupLimit,
    pub dir: BackupLimit,
}

#[derive(Serialize, Deserialize)]
pub struct TileClass {
    pub description: String,
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: TileProperties,
    #[serde(rename = "type")]
    pub tile_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct TileProperties {
    pub t: BackupLimit,
    pub d: Pattern,
    pub px: Pattern,
    pub f: BackupLimit,
    pub src: Pattern,
}

#[derive(Serialize, Deserialize)]
pub struct TilesetDef {
    pub description: String,
    pub title: String,
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
    pub properties: TilesetDefProperties,
    #[serde(rename = "type")]
    pub tileset_def_type: Vec<BackupLimitType>,
}

#[derive(Serialize, Deserialize)]
pub struct TilesetDefProperties {
    #[serde(rename = "cachedPixelData")]
    pub cached_pixel_data: BackupLimit,
    #[serde(rename = "__cHei")]
    pub c_hei: BackupLimit,
    #[serde(rename = "pxHei")]
    pub px_hei: BackupLimit,
    #[serde(rename = "customData")]
    pub custom_data: Pattern,
    #[serde(rename = "tagsSourceEnumUid")]
    pub tags_source_enum_uid: BackupLimit,
    pub uid: BackupLimit,
    pub padding: BackupLimit,
    #[serde(rename = "enumTags")]
    pub enum_tags: Pattern,
    #[serde(rename = "pxWid")]
    pub px_wid: BackupLimit,
    #[serde(rename = "__cWid")]
    pub c_wid: BackupLimit,
    pub spacing: BackupLimit,
    pub identifier: BackupLimit,
    #[serde(rename = "savedSelections")]
    pub saved_selections: Pattern,
    #[serde(rename = "relPath")]
    pub rel_path: BackupLimit,
    #[serde(rename = "tileGridSize")]
    pub tile_grid_size: BackupLimit,
}

#[derive(Serialize, Deserialize)]
pub enum BackupLimitType {
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "string")]
    String,
}

#[derive(Serialize, Deserialize)]
pub enum FlagsType {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "null")]
    Null,
}