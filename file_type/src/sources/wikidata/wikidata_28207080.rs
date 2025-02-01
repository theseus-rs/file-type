use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207080: FileFormat = FileFormat {
    id: 28_207_080,
    puid: "wikidata/28207080",
    name: "PrintMaster Names file",
    extensions: &["sdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
