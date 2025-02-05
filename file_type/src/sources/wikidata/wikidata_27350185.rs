use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27350185: FileFormat = FileFormat {
    id: 27_350_185,
    source_type: SourceType::Wikidata,
    name: "ADRG Test Patch Image File",
    extensions: &["cph"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
