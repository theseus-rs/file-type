use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122583982: FileFormat = FileFormat {
    id: 122_583_982,
    source_type: SourceType::Wikidata,
    name: "Zetafax Fax Image File (Normal)",
    extensions: &["g3n"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
