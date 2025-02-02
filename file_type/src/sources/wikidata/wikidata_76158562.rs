use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_76158562: FileFormat = FileFormat {
    id: 76_158_562,
    source_type: SourceType::Wikidata,
    name: "VisKit 3d model",
    extensions: &["vk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
