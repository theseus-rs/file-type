use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131994277: FileFormat = FileFormat {
    id: 131_994_277,
    puid: "wikidata/131994277",
    name: "Web Archive Transformation",
    extensions: &["wat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
