use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110995856: FileFormat = FileFormat {
    id: 110_995_856,
    puid: "wikidata/110995856",
    name: "PrintMaster Card File format",
    extensions: &["car"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
