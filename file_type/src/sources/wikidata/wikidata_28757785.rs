use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757785: FileFormat = FileFormat {
    id: 28_757_785,
    puid: "wikidata/28757785",
    name: "GMLJP2",
    extensions: &["jpf", "jpx"],
    media_types: &["image/jpx", "image/jpx"],
    internal_signatures: &[],
    related_formats: &[],
};
