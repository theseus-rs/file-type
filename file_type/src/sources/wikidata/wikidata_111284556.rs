use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111284556: FileFormat = FileFormat {
    id: 111_284_556,
    source_type: SourceType::Wikidata,
    name: "GigaStudio/GigaSampler file",
    extensions: &["gi!", "gig"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
