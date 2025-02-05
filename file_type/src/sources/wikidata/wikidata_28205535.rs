use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205535: FileFormat = FileFormat {
    id: 28_205_535,
    source_type: SourceType::Wikidata,
    name: "iThmb",
    extensions: &["ithmb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
