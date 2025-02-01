use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205679: FileFormat = FileFormat {
    id: 28_205_679,
    puid: "wikidata/28205679",
    name: "Amber ARR Bitmap Image",
    extensions: &["arr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
