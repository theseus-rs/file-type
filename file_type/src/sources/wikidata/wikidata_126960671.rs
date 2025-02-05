use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126960671: FileFormat = FileFormat {
    id: 126_960_671,
    source_type: SourceType::Wikidata,
    name: "Vala source file",
    extensions: &["vala", "vapi"],
    media_types: &["text/x-vala"],
    signatures: &[],
    related_formats: &[],
};
