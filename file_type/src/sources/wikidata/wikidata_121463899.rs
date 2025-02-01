use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121463899: FileFormat = FileFormat {
    id: 121_463_899,
    puid: "wikidata/121463899",
    name: "Adobe Lightroom Database",
    extensions: &["lrdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
