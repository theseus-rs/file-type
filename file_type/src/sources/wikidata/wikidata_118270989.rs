use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118270989: FileFormat = FileFormat {
    id: 118_270_989,
    source_type: SourceType::Wikidata,
    name: "Altamira Composer file",
    extensions: &["acc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
