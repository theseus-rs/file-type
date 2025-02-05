use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122073134: FileFormat = FileFormat {
    id: 122_073_134,
    source_type: SourceType::Wikidata,
    name: "MidiScan File",
    extensions: &["mnd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
