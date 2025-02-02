use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_56827141: FileFormat = FileFormat {
    id: 56_827_141,
    source_type: SourceType::Wikidata,
    name: "SeqBox file",
    extensions: &["sbx"],
    media_types: &["application/x-sbx"],
    internal_signatures: &[],
    related_formats: &[],
};
