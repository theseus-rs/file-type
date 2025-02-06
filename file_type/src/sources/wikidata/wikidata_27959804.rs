use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959804: FileFormat = FileFormat {
    id: 27_959_804,
    source_type: SourceType::Wikidata,
    name: "Ableton Live Clip",
    extensions: &["alc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
