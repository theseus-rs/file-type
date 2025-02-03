use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1196547: FileFormat = FileFormat {
    id: 1_196_547,
    source_type: SourceType::Wikidata,
    name: "Design Web Format",
    extensions: &["dwf", "dwfx"],
    media_types: &["model/vnd-dwf"],
    internal_signatures: &[],
    related_formats: &[],
};
