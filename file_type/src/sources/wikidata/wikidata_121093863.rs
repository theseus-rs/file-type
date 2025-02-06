use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121093863: FileFormat = FileFormat {
    id: 121_093_863,
    source_type: SourceType::Wikidata,
    name: "Punch! Home Suite PHD file",
    extensions: &["phd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
