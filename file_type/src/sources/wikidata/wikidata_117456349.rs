use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117456349: FileFormat = FileFormat {
    id: 117_456_349,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Database File 1.0",
    extensions: &["mda", "mdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
