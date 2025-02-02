use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121093863: FileFormat = FileFormat {
    id: 121_093_863,
    source_type: SourceType::Wikidata,
    name: "Punch! Home Suite PHD file",
    extensions: &["phd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
