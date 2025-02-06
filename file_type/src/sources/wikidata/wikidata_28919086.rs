use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28919086: FileFormat = FileFormat {
    id: 28_919_086,
    source_type: SourceType::Wikidata,
    name: "CMX 3600 edit decision list",
    extensions: &["edl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
