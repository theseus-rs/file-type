use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2375766: FileFormat = FileFormat {
    id: 2_375_766,
    puid: "wikidata/2375766",
    name: "Synchronized Accessible Media Interchange",
    extensions: &["sami", "smi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
