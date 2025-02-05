use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979406: FileFormat = FileFormat {
    id: 27_979_406,
    source_type: SourceType::Wikidata,
    name: "QTL",
    extensions: &["qtl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
