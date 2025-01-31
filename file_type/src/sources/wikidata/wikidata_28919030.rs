use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919030: FileFormat = FileFormat {
    id: 28_919_030,
    puid: "wikidata/28919030",
    name: "AC-3 Compressed Audio (Dolby Digital), Revision A",
    extensions: &["ac3"],
    media_types: &["audio/ac3"],
    internal_signatures: &[],
    related_formats: &[],
};
