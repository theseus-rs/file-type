use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131179431: FileFormat = FileFormat {
    id: 131_179_431,
    source_type: SourceType::Wikidata,
    name: "TableGen file format",
    extensions: &["td"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
