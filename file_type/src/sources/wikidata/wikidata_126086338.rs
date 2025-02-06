use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126086338: FileFormat = FileFormat {
    id: 126_086_338,
    source_type: SourceType::Wikidata,
    name: "IMF Package Packing List",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
