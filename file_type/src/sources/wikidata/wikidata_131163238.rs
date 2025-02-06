use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131163238: FileFormat = FileFormat {
    id: 131_163_238,
    source_type: SourceType::Wikidata,
    name: "Stan model file",
    extensions: &["stan"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
