use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131179431: FileFormat = FileFormat {
    id: 131_179_431,
    source_type: SourceType::Wikidata,
    name: "TableGen file format",
    extensions: &["td"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
