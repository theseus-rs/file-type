use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111284556: FileFormat = FileFormat {
    id: 111_284_556,
    source_type: SourceType::Wikidata,
    name: "GigaStudio/GigaSampler file",
    extensions: &["gi!", "gig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
