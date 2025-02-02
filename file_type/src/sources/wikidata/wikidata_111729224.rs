use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111729224: FileFormat = FileFormat {
    id: 111_729_224,
    source_type: SourceType::Wikidata,
    name: "Document Manager file",
    extensions: &["ddm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
