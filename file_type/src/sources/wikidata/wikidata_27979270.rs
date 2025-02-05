use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979270: FileFormat = FileFormat {
    id: 27_979_270,
    source_type: SourceType::Wikidata,
    name: "TheDraw Save File",
    extensions: &["td"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
