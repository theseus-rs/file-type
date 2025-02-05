use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125925206: FileFormat = FileFormat {
    id: 125_925_206,
    source_type: SourceType::Wikidata,
    name: "Papyrus Author database",
    extensions: &["pb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
