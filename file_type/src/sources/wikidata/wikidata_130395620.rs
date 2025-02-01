use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130395620: FileFormat = FileFormat {
    id: 130_395_620,
    puid: "wikidata/130395620",
    name: "Octave source code file",
    extensions: &["m"],
    media_types: &["text/octave"],
    internal_signatures: &[],
    related_formats: &[],
};
