# Step 0: Become familiar with Rust basics

## What memory model [Rust] has? Is it single-threaded or multiple-threaded? Is it synchronous or asynchronous?

Rust не имеет определенной модели памяти, но над этим идет работа. Rust
поддерживает многопоточность ([`thread::spawn` в стандартной библиотеке](https://doc.rust-lang.org/stable/std/thread/fn.spawn.html)) и
асинхронность программ (модули [`task`](https://doc.rust-lang.org/stable/std/task/index.html), [`future`](https://doc.rust-lang.org/stable/std/future/index.html)).

## What runtime [Rust] has? Does it use a GC (garbage collector)?

Rust не имеет рантайма в привычном понимании, но он предоставляет использование
памяти в куче, backtrace, разматывание стека и stack guards. Стандартная
библиотека Rust [инициализирует рантайм](https://github.com/rust-lang/rust/blob/33916307780495fe311fe9c080b330d266f35bfb/src/libstd/rt.rs#L43)
перед запуском функции main, куда входит инициализация panic handler'а, сбор
аргументов командной строки, обработчик сигналов, запуск main потока. Инициализацию
рантайма можно отключить, используя `#![no_std]` атрибут (что потребует ручную
инициализацию обработчика паники). У него нет сборщика мусора, в отличие от
языков, таких как Java или Go. Rust использует систему
владения и заимствования для управления ресурсами, примерно как [RAII] (Resource Acquisition Is Initialization)
в C++: деструктор объекта запускается, когда объект пропадает из области
видимости (дропается) в конце блока кода, позволяя освободить память, закрыть
ресурс и т.д.

## What statically typing means? What is a benefit of using it?

Статическая типизация -- характеристика языка программирования. При статической
типизации, каждая переменная, параметр и возвращаемое значение функции, имеют
определенный тип, объявленный в момент определения, который не может быть
изменен позже, в рантайме. Это позволяет проверять все типы, используемые в
программе, и их связи, в процессе компиляции; проводить оптимизации кода,
используя полученную информацию о переменных и функциях.

## What are generics and parametric polymorphism? Which problems do they solve?

Generics (обобщенные типы) -- типовые параметры либо параметры времени жизни.
Параметрический полиморфизм в отличие от перегрузки функций и
ad-hoc-полиморфизма, при котором требуется реализовывать функции отдельно для
каждого типа, с которым она может использоваться, либо полагаться на приведение
типов, это способ обрабатывать значения разных типов идентичным способом.
Функции, типы (структуры, перечисления, юнионы), трейты, псевдонимы типов,
`impl` блоки могут иметь генерик параметры. Генерики позволяют использовать одни
и те же типы данных/функции с различными параметрами. Rust использует технику
мономорфизации генерик кода: при инициализации типов или использовании функций,
будут статически определены используемые типы и будет создана копия генерик
функции/типа с конкретными типами.

Генерики объявляются с помощью синтаксиса с треугольными скобками `<T, U>`.
Трейт баунды (ограничения) на генерик типы пишутся после `:` и перечисляются
через `+`: `<T: Clone + Sized>`. С помощью синтаксиса с `where` можно назначить
ограничения на любые типы.

## What are traits? How are they used? How do they compare to interfaces? What are an auto trait and a blanket impl? What is a marker trait?

Трейты (типажи) описывают интерфейс для типов, которые могут его реализовать.
Трейт может иметь ассоциированные айтемы: функции, типы, константы, которые
должны быть определены реализующим трейт типом.

Генерик типы/функции могут использовать трейты для ограничения типов, которые
могут быть переданы на место параметров. Отличие трейтов от интерфейсов в других
языках в том, что трейты могут быть реализованы для любых типов, в то время как
реализация интерфейсов должна находиться в месте определения типа. Таким образом
можно реализовывать трейты любым другим типам, к примеру используя генеричный
`impl` блок: `impl<T> Trait for T { ... }`, что называется "blanket impl".

Маркер трейт -- трейт без ассоциированных айтемов.

[Авто трейты](https://github.com/rust-lang/rust/issues/13231) -- специальные маркер трейты, реализация которых генерируется компилятором
автоматически и может быть отменена используя [отрицательный `impl`](https://github.com/rust-lang/rust/issues/68318).

## What are static and dynamic dispatches? Which should I use, and when?

Статическая диспетчеризация может быть объяснена на примере генериков и
мономорфизации. При мономорфизации функций компилятор генерирует функции для
каждого уникального использования функции с различными типами на месте
генериков, что позволяет в дальнейшем оптимизировать код, используя инлайнинг.
Но такой способ чреват "раздутием" выходного бинарного файла, т.к. он будет
содержать реализацию для каждой генерик функции (которая не была заинлайнена), а
также замедлением компиляции.

Другой пример -- гетерогенные коллекции. Когда есть необходимость сложить в
вектор значения разных типов, мы можем использовать статическую диспетчеризацию,
создав `enum`, который может содержать объекты данных типов. Но в таком случае
возможно использование лишь ограниченного количества типов (определенных в
перечислении).

Чтобы решить проблемы, описанные выше, можно использовать динамическую
диспетчеризацию. Достигается это путем использования динамических объектов вида
`dyn Trait`, где `Trait` -- определенный
[object-safe](https://rust-lang.github.io/rfcs/0255-object-safety.html) трейт.
Функции и типы могут принимать динамические объекты как параметры. Динамические объекты
должны храниться за некоторого рода указателем, к примеру, `&dyn Trait`. Такой
тип состоит из двух указателей: на оригинальный объект и на таблицу виртуальных
функций соответствующего трейта. Таким образом, вызов соответствующей функции
будет происходить в рантайме.

Следует использовать статическую диспетчеризацию когда это возможно, особенно в
"горячих" функциях т.к. это позволяет получать более быстрый код. Динамическая
диспетчеризация полезна, когда коду, вызывающему методы трейта, не известны все
типы, которые могут быть использованы (к примеру, библиотека использует
реализации трейта для юзер типов), а важно лишь наличие имплементации.

## What is a crate and what is a module in Rust? How do they differ? How are the used?

Крейт -- единица компиляции в Rust. В основном крейты бывают двух типов:
библиотека и бинарный файл. Крейт библиотеки содержит в себе реализации всех
генерик функций и типов, определенных в коде библиотеки (для дальнейшего
использования при мономорфизации кода). Бинарный (исполняемый) крейт -- таковой,
который определяет функцию `main` как точку входа программы. Существуют также
`proc macro` крейты, определяющие процедурные макросы; `cdylib` крейты,
компилируемые в динамическую библиотеку.

Модуль -- единица организации кода в крейте. Модули могут быть вложенными и
образуют из себя дерево модулей. Модули содержат в себе определения айтемов и
реализации трейтов, образуя крейт вместе с неявно определенным модулем верхнего
уровня `crate`.

Не следует путать крейты Rust с пакетами Cargo. Терминология Cargo называет
пакетом набор крейтов, включающих в себя несколько бинарных крейтов и/или один
крейт библиотеки.

## What are move semantics? What are borrowing rules? What is the benefit of using them?

Семантика перемещения, семантика владения и заимствования - ключевые явления
системы типов Rust. Есть три правила владения:

- Каждое значение имеет переменную, называемую "владельцем".
- В любой момент времени у значения существует лишь один владелец.
- Когда переменная выходит из области видимости, ее значение будет дропнуто.

При присвоении одной переменной -- другой, при передаче переменной в параметры
функции, при инициализации поля структуры/юниона/енума, значение будет
"перемещено" и старая переменная будет не валидна. Компилятор следит за
использованием переменных, их инициализацией и инвалидированием.

Вся эта система нужна для избежания проблем использования и закрытия ресурсов:
памяти, открытых файлов и т.д. Когда значением может владеть лишь одна
переменная, а также она будет дропнута в конце области видимости, мы можем быть
уверены, что не произойдет таких ошибок, как Use After Free, Double Free, Memory
Leak [^1].

Заимствование -- создание ссылки на значение другой переменной. Существует два
вида ссылок: shared (общие), которые можно использовать лишь для чтения
значения, и mutable (или mutually exclusive), с помощью которых можно менять
ссылаемое значение. Есть три правила ссылок (заимствования):

- Для значения может существовать либо N shared ссылок, либо одна mutable ссылка.
- Все ссылки всегда валидные (указывают на инициализированное значение).
- Ссылки на значение не могут пережить владельца значения.

Благодаря этим правилам мы избавляемся от ошибок гонки данных, которые могут
случаться в других языках без такой системы, когда два или более указателя имеют
доступ к некоторому значению одновременно, хотя бы один из них может изменять
данные и нет никаких средств синхронизации доступа. За использованием ссылок
следит borrow checker.

## What is immutability? What is the benefit of using it?

По умолчанию все созданные переменные в Rust неизменяемые, но могут быть
объявленными изменяемыми с помощью ключевого слова `mut` в `let mut x = ...;` и
`fn foo(mut x: ...) {}`.

## What is cloning? What is copying? How do they compare?

Любой тип (для которого это имеет смысл) может имплементировать трейт
[Clone](https://doc.rust-lang.org/stable/std/clone/trait.Clone.html), требующий
реализации метода с сигнатурой `fn clone(&self) -> Self`, т.е., данным методом
тип должен возвращать "клон" значения `self`.

`Clone`-типы могут также имплементировать специальный маркер трейт
[`Copy`](https://doc.rust-lang.org/stable/std/marker/trait.Copy.html). Смотря на
такой тип, компилятор не будет инвалидировать старых владельцев значения на
местах перемещения владения, а лишь скопирует значение в новую переменную (не
вызывая `Clone::clone`, а просто копируя байты прежнего значения).

## What is RAII? How is it implemented in [Rust]? What is the benefit of using it?

[RAII] - идиома из языка C++, связывающая создание/получение ресурса (открытие
файла, аллокация памяти и т.д.) с созданием объекта и освобождение ресурса
(освобождение памяти, закрытие файла...) с удалением объекта.

В Rust это реализовано с помощью специального трейта
[`Drop`](https://doc.rust-lang.org/stable/std/ops/trait.Drop.html), метод `drop`
которого автоматически вызывается компилятором, когда переменная выходит из
области видимости (достигает конца блока) или когда происходит разматывание
стека при панике. `drop` принимает мутабельную ссылку на дропаемый объект,
позволяя сделать последние манипуляции с ресурсами, связанными с объектом,
прежде чем он станет более недоступен.

Данная идиома предоставляет возможность инкапсулирования логики управления
ресурсом в создании/удалении объекта. В связке с системой владения Rust, это
хороший инструмент для автоматического управления ресурсами с гарантиями от
языка.

## What is an iterator? What is a collection? How do they differ? How are they used?

Итератор -- тип, реализующий трейт [`Iterator`], требующий реализации метода
`next`, определения ассоциированного типа `Item` -- тип элементов,
производящихся итератором и предоставляющий множество методов для создания
итератор адаптеров, меняющих поведение итератора. Метод `next` возвращает
`Option<T>`, сигнализируя об окончании итерации, возвращая `None`.

Коллекции -- структуры данных, предназначенные для хранения многих значений
некоторых типов. Примеры коллекций, реализованных в стандартной библиотеке:
[`Vec<T>`], [`HashMap<K, V>`], [`LinkedList<T>`] и другие.

Отличие итераторов от коллекций в том, что итераторы _ленивые_ и производят
новый элемент когда был вызван `next`, в то время как коллекция -- это набор
элементов (зачастую, аллоцированных в куче).

Коллекции реализуют трейт итератора и [`IntoIterator`] для возможности удобной
итерации по элементам коллекции.

Использовать итератор можно:

1. Просто вызывая у него метод `next`: `it.next();` в цикле:
   1. `loop`/`while`:

      ```rust
      loop {
        let elem = it.next();
      }
      ```

   2. `while let`:

      ```rust
      while let Some(elem) = it.next() {
      }
      ```

2. В цикле `for`[^2]:

    ```rust
    for elem in it {
    }
    ```

3. Вызывая у него другие методы итератора, трансформирующие итератор:

   ```rust
   it.take(10).skip(3).map(foo).collect()
   ```

## What are macros? Which problems do they solve? What is the difference between declarative and procedural macro?

Макросы -- определяемые расширения синтаксиса языка.

Макросы вызываются с помощью синтаксиса `macro_name!()`, принимают на вход
произвольный[^3] набор токенов Rust и раскрываются в набор токенов. Раскрытие
макросов происходит на этапе компиляции во время парсинга кода (в три этапа:
парсинг кода, определение макросов и раскрытие макросов с последующим парсингом
нового дерева токенов).

Макросы предоставляют возможность разработчиком определять свои расширения языка
или даже DSL. Часто используются для сокращения повторения кода.

Декларативные макросы или "macro by example" -- макросы, определенные с помощью
синтаксиса `macro_rules!`, в декларативном стиле, и имеют вид "паттерн матчинга"
с примерами наборов токенов и того, во что должен раскрыться конкретный набор
токенов.

Процедурные макросы -- макросы в императивном стиле, которые должны быть
определены в отдельном крейте с типом `proc-macro`.

Отличия процедурных макросов от декларативных:

- Гигиена: при создании нового имени (переменной) в раскрытии декларативного
  макроса, имя будет изменено, чтобы не конфликтовать с именами из окружения. В
  процедурных же макросах гигиена не работает.
- Декларативные макросы принимают на вход правильное дерево токенов Rust.
  Процедурные макросы могут принимать на вход любые токены (значение типа
  `proc_macro::TokenStream`).

Процедурные макросы могут быть одним из трех видов:

1. Derive macro. Данные макросы существуют для использования в атрибуте
   `#[derive(..)]` и предназначены для автоматического создания имплементации
   некоторого трейта.
2. Attribute macro. Макросы-атрибуты имеют вид `#[attr]` и могут быть
   использованы перед определениями функций, структур, енумов, юнионов, трейтов,
   реализациями трейтов.
3. Function-like macro. Функциональные макросы имеют вид обычных макросов:
   `macro_name!()`

## How code is tested in [Rust]? Where should you put tests and why?

Rust имеет встроенный функционал для написания тестов. Это достигается
использованием атрибута `#[test]` у функций и условной компиляции тестов с
помощью `#[cfg(test)]`.

Разделяют два вида тестов: юнит тесты и интеграционные тесты.

Юнит тесты помещаются рядом с объектом тестирования и обычно определяются в
модуле

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_something() {}
}
```

Такие тесты имеют видимость приватных определений крейта и удобны для
тестирования "внутренностей" работы библиотек.

Интеграционные же тесты помещаются в отдельную папку в проекте (tests по
умолчанию) и компилируются Cargo как отдельные крейты. Данные тесты полезны для
тестирования интерфейса библиотеки.

Cargo предоставляет команду `cargo test` для запуска и управления тестами.

## Why [Rust] has `&str` and `String` types? How do they differ? When should you use them?

Тип `String` -- структура, определенная в стандартной библиотеке. `String`
хранит в себе `Vec<u8>` -- вектор байт. Иными словами, `String` -- владеющий
тип. Байты строки аллоцируются в куче. `String` имеет длину и вместимость (как и
вектор). Строки могут быть изменены в длине.

Тип `&str` -- _слайс_ строки, работает как `&[u8]`. Слайс строки -- заимствующий
тип, он лишь ссылается на строку, но не владеет ею. `&str` хранит в себе длину
строки. `&str` не может быть изменен в длине.

`&str`, так как это ссылка, имеет и вид `&mut str`, через который можно изменять
контент строки, но не добавлять/удалять из нее байты.

Строки, записанные в виде литералов, имеют тип `&'static str` и хранятся в
read-only секции бинарного файла.

## What are lifetimes? Which problems do they solve? Which benefits do they give?

Лайфтаймы, или времена жизни ссылок, это то, что позволяет компилятору (а точнее
его borrow checker) определять, что все ссылки валидны. Каждой ссылке borrow
checker назначает соответствующее время жизни. Обозначается как `'lt`.

Лайфтаймы служат для выражения взаимоотношений между ссылками. Вам может
понадобиться описать их в сигнатуре функций или типов. При выводе времен жизни,
borrow checker руководствуется такими правилами:

1. Каждая ссылка в аргументах функции получает свой лайфтайм.
2. Если в функции всего одна ссылка в аргументах, возвращаемое значение получает
   такой же лайфтайм.
3. Если первый параметр -- ссылка на `self`, выходной параметр получает его
   время жизни.

Иначе, компилятор выдаст ошибку и потребуется вручную описать лайфтаймы в
функции.

## Is [Rust] OOP language? Is it possible to use SOLID/GRASP? Does it have an inheritance?

Rust -- мультипарадигмальный. Он может считаться ОО языком по некоторым
определениям. Многое, что можно делать в ОО языках, можно так или иначе
повторить в Rust, используя структуры и трейты. Rust не имеет наследования.

[Cargo]: https://github.com/rust-lang/cargo
[Cargo Book]: https://doc.rust-lang.org/cargo
[Rust]: https://www.rust-lang.org
[Rust Book]: https://doc.rust-lang.org/book
[Rust By Example]: https://doc.rust-lang.org/rust-by-example
[Rust FAQ]: https://www.rust-lang.org/faq.html
[RAII]: https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization
[`Vec<T>`]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
[`HashMap<K, V>`]: https://doc.rust-lang.org/stable/std/collections/hash_map/struct.HashMap.html
[`LinkedList<T>`]: https://doc.rust-lang.org/stable/std/collections/struct.LinkedList.html
[`Iterator`]: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
[`IntoIterator`]: https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html#
[^1]: Кроме случаев использования
    [`mem::forget`](https://doc.rust-lang.org/stable/std/mem/fn.forget.html),
    цикличной зависимости
    [`Rc`](https://doc.rust-lang.org/stable/std/rc/struct.Rc.html) и др.
[^2]: Цикл `for` рассахаривается в

    ```rust
    {
        let it = it.into_iter();
        loop {
            match it.next() {
                Some(elem) => { /* body */ }
                None => break
            }
        }
    }
    ```
[^3]: валидное дерево токенов -- правильную скобочную последовательность.
