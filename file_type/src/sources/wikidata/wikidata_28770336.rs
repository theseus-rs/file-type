use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28770336: FileFormat = FileFormat {
    id: 28_770_336,
    source_type: SourceType::Wikidata,
    name: "Lotus 1-2-3 Chart",
    extensions: &["pic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
