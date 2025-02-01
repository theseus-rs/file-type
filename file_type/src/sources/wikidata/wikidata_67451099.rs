use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67451099: FileFormat = FileFormat {
    id: 67_451_099,
    puid: "wikidata/67451099",
    name: "IBM Softcopy Reader (Bookmanager) Bookshelf (and Book) index file",
    extensions: &["bki"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
