use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111665313: FileFormat = FileFormat {
    id: 111_665_313,
    source_type: SourceType::Wikidata,
    name: "AbiWord Gzip Compressed Document",
    extensions: &["zabw"],
    media_types: &["application/abiword"],
    internal_signatures: &[],
    related_formats: &[],
};
