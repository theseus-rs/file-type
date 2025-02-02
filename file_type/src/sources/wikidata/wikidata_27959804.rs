use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27959804: FileFormat = FileFormat {
    id: 27_959_804,
    source_type: SourceType::Wikidata,
    name: "Ableton Live Clip",
    extensions: &["alc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
