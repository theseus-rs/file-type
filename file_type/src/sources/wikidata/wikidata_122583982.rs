use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122583982: FileFormat = FileFormat {
    id: 122_583_982,
    source_type: SourceType::Wikidata,
    name: "Zetafax Fax Image File (Normal)",
    extensions: &["g3n"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
