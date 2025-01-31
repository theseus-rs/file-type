use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122021046: FileFormat = FileFormat {
    id: 122_021_046,
    puid: "wikidata/122021046",
    name: "Retina file",
    extensions: &["rtd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
