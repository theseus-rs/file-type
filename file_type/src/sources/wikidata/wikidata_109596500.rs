use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109596500: FileFormat = FileFormat {
    id: 109_596_500,
    source_type: SourceType::Wikidata,
    name: "DrawPlus Animation",
    extensions: &["dpa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
