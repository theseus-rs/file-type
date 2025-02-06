use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111166086: FileFormat = FileFormat {
    id: 111_166_086,
    source_type: SourceType::Wikidata,
    name: "Songbase File",
    extensions: &["sngbase"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
