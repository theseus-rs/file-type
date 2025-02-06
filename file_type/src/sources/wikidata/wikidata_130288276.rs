use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130288276: FileFormat = FileFormat {
    id: 130_288_276,
    source_type: SourceType::Wikidata,
    name: "MYTHSAV",
    extensions: &["mythsav"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
