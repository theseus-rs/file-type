use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122073134: FileFormat = FileFormat {
    id: 122_073_134,
    puid: "wikidata/122073134",
    name: "MidiScan File",
    extensions: &["mnd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
