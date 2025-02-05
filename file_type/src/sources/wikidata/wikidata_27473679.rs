use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27473679: FileFormat = FileFormat {
    id: 27_473_679,
    source_type: SourceType::Wikidata,
    name: "Band Sequential Image File",
    extensions: &["bsq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
