use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600228: FileFormat = FileFormat {
    id: 28_600_228,
    source_type: SourceType::Wikidata,
    name: "APL workspace",
    extensions: &["apl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
