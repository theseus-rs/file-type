use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60479192: FileFormat = FileFormat {
    id: 60_479_192,
    puid: "wikidata/60479192",
    name: "Quattro Pro Spreadsheet for Windows",
    extensions: &["wb1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
