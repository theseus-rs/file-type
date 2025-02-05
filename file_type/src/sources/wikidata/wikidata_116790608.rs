use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116790608: FileFormat = FileFormat {
    id: 116_790_608,
    source_type: SourceType::Wikidata,
    name: "InDesign template",
    extensions: &["indt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
