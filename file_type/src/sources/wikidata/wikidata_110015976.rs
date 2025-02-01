use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110015976: FileFormat = FileFormat {
    id: 110_015_976,
    puid: "wikidata/110015976",
    name: "MIG Graphics File",
    extensions: &["mig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
