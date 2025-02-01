use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28771271: FileFormat = FileFormat {
    id: 28_771_271,
    puid: "wikidata/28771271",
    name: "MSA (Magic Shadow Archiver)",
    extensions: &["msa"],
    media_types: &["application/vnd.msa-disk-image"],
    internal_signatures: &[],
    related_formats: &[],
};
