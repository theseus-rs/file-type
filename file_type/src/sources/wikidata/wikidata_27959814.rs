use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959814: FileFormat = FileFormat {
    id: 27_959_814,
    source_type: SourceType::Wikidata,
    name: "Ableton Meta Sound",
    extensions: &["ams"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
