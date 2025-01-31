use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121760718: FileFormat = FileFormat {
    id: 121_760_718,
    puid: "wikidata/121760718",
    name: "Adobe Acrobat MIME Encoded Job Definition File",
    extensions: &["mjd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
