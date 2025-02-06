use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111009741: FileFormat = FileFormat {
    id: 111_009_741,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Fax Cover File format",
    extensions: &["fax"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
