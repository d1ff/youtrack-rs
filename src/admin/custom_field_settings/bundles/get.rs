imports!();

use crate::admin::custom_field_settings::get::CustomFieldSettings;

new_type!(
    Bundles
    BundlesEnum
    BundlesEnumTop
    BundlesEnumTopSkip
    BundlesEnumTopSkipFields
    BundlesEnumBundleId
    BundlesEnumBundleIdFields
    BundlesEnumBundleIdValues
    BundlesEnumBundleIdValuesFields
    BundlesState
    BundlesStateTop
    BundlesStateTopSkip
    BundlesStateTopSkipFields
    BundlesStateBundleId
    BundlesStateBundleIdFields
    BundlesStateBundleIdValues
    BundlesStateBundleIdValuesFields);

from!(
    @CustomFieldSettings
    -> Bundles = "bundles"
    @Bundles
    -> BundlesEnum = "enum"
    @BundlesEnum
    => BundlesEnumBundleId
    @BundlesEnum
    ?> BundlesEnumTop = "$top"
    @BundlesEnumTop
    ?> BundlesEnumTopSkip = "$skip"
    @BundlesEnumTopSkip
    ?> BundlesEnumTopSkipFields = "fields"
    @BundlesEnumBundleId
    -> BundlesEnumBundleIdValues = "values"
    @BundlesEnumBundleId
    ?> BundlesEnumBundleIdFields = "fields"
    @BundlesEnumBundleIdValues
    ?> BundlesEnumBundleIdValuesFields = "fields"
    @Bundles
    -> BundlesState = "state"
    @BundlesState
    => BundlesStateBundleId
    @BundlesState
    ?> BundlesStateTop = "$top"
    @BundlesStateTop
    ?> BundlesStateTopSkip = "$skip"
    @BundlesStateTopSkip
    ?> BundlesStateTopSkipFields = "fields"
    @BundlesStateBundleId
    -> BundlesStateBundleIdValues = "values"
    @BundlesStateBundleId
    ?> BundlesStateBundleIdFields = "fields"
    @BundlesStateBundleIdValues
    ?> BundlesStateBundleIdValuesFields = "fields");

impl_macro!(
    @Bundles
    |=> enum_ -> BundlesEnum
    |=> state -> BundlesState
    |
    @BundlesEnum
    |
    |=> id -> BundlesEnumBundleId = id_str
    |?> top -> BundlesEnumTop = top
    @BundlesEnumTop
    |
    |?> skip -> BundlesEnumTopSkip = skip
    @BundlesEnumTopSkip
    |
    |?> fields -> BundlesEnumTopSkipFields = fields
    @BundlesEnumBundleId
    |=> values -> BundlesEnumBundleIdValues
    |
    @BundlesEnumBundleId
    |
    |?> fields -> BundlesEnumBundleIdFields = fields
    @BundlesEnumBundleIdValues
    |
    |?> fields -> BundlesEnumBundleIdValuesFields = fields
    @BundlesState
    |
    |=> id -> BundlesStateBundleId = id_str
    |?> top -> BundlesStateTop = top
    @BundlesStateTop
    |
    |?> skip -> BundlesStateTopSkip = skip
    @BundlesStateTopSkip
    |
    |?> fields -> BundlesStateTopSkipFields = fields
    @BundlesStateBundleId
    |=> values -> BundlesStateBundleIdValues
    |
    @BundlesStateBundleId
    |
    |?> fields -> BundlesStateBundleIdFields = fields
    @BundlesStateBundleIdValues
    |
    |?> fields -> BundlesStateBundleIdValuesFields = fields);

exec!(BundlesEnumTopSkipFields);
exec!(BundlesEnumBundleIdFields);
exec!(BundlesEnumBundleIdValuesFields);
exec!(BundlesStateTopSkipFields);
exec!(BundlesStateBundleIdFields);
exec!(BundlesStateBundleIdValuesFields);
